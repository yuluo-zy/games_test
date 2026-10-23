#!/usr/bin/env python3
"""Real X11/XTest input and WM_DELETE_WINDOW; never opens save/config files."""
import argparse
import ctypes as C
import ctypes.util
import json
import os
import time

X = C.CDLL(ctypes.util.find_library('X11'))
T = C.CDLL(ctypes.util.find_library('Xtst'))
D = C.c_void_p
W = C.c_ulong
X.XOpenDisplay.argtypes = [C.c_char_p]
X.XOpenDisplay.restype = D
X.XDefaultRootWindow.argtypes = [D]
X.XDefaultRootWindow.restype = W
X.XQueryTree.argtypes = [D, W, C.POINTER(W), C.POINTER(W), C.POINTER(C.POINTER(W)), C.POINTER(C.c_uint)]
X.XFetchName.argtypes = [D, W, C.POINTER(C.c_void_p)]
X.XGetGeometry.argtypes = [D, W, C.POINTER(W), C.POINTER(C.c_int), C.POINTER(C.c_int), C.POINTER(C.c_uint), C.POINTER(C.c_uint), C.POINTER(C.c_uint), C.POINTER(C.c_uint)]
X.XTranslateCoordinates.argtypes = [D, W, W, C.c_int, C.c_int, C.POINTER(C.c_int), C.POINTER(C.c_int), C.POINTER(W)]
X.XFree.argtypes = [C.c_void_p]
X.XStringToKeysym.argtypes = [C.c_char_p]
X.XStringToKeysym.restype = C.c_ulong
X.XKeysymToKeycode.argtypes = [D, C.c_ulong]
X.XKeysymToKeycode.restype = C.c_uint
X.XSetInputFocus.argtypes = [D, W, C.c_int, C.c_ulong]
X.XRaiseWindow.argtypes = [D, W]
X.XSync.argtypes = [D, C.c_int]
X.XCloseDisplay.argtypes = [D]
X.XInternAtom.argtypes = [D, C.c_char_p, C.c_int]
X.XInternAtom.restype = C.c_ulong
T.XTestFakeKeyEvent.argtypes = [D, C.c_uint, C.c_int, C.c_ulong]
T.XTestFakeButtonEvent.argtypes = [D, C.c_uint, C.c_int, C.c_ulong]
T.XTestFakeMotionEvent.argtypes = [D, C.c_int, C.c_int, C.c_int, C.c_ulong]

class Data(C.Union):
    _fields_ = [('b', C.c_char * 20), ('s', C.c_short * 10), ('l', C.c_long * 5)]

class Client(C.Structure):
    _fields_ = [('type', C.c_int), ('serial', C.c_ulong), ('send_event', C.c_int), ('display', D), ('window', W), ('message_type', C.c_ulong), ('format', C.c_int), ('data', Data)]

class Event(C.Union):
    _fields_ = [('client', Client), ('pad', C.c_long * 24)]

X.XSendEvent.argtypes = [D, W, C.c_int, C.c_long, C.POINTER(Event)]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=['list', 'inspect', 'focus', 'move', 'click', 'key', 'close'])
parser.add_argument('--window', type=lambda value: int(value, 0))
parser.add_argument('--x', type=int)
parser.add_argument('--y', type=int)
parser.add_argument('--key', default='Return', help='X keysym, e.g. Return, Tab, Escape')
parser.add_argument('--button', type=int, default=1)
parser.add_argument('--hold', type=float, default=0.08)
args = parser.parse_args()
display = X.XOpenDisplay(None)
if not display:
    raise SystemExit('Cannot open DISPLAY=' + os.getenv('DISPLAY', ''))
root = X.XDefaultRootWindow(display)

def inspect(window):
    name_pointer = C.c_void_p()
    title = ''
    if X.XFetchName(display, window, C.byref(name_pointer)) and name_pointer.value:
        title = C.string_at(name_pointer.value).decode(errors='replace')
        X.XFree(name_pointer)
    gr, gx, gy = W(), C.c_int(), C.c_int()
    width, height, border, depth = (C.c_uint() for _ in range(4))
    X.XGetGeometry(display, window, C.byref(gr), C.byref(gx), C.byref(gy), C.byref(width), C.byref(height), C.byref(border), C.byref(depth))
    x, y, child = C.c_int(), C.c_int(), W()
    X.XTranslateCoordinates(display, window, root, 0, 0, C.byref(x), C.byref(y), C.byref(child))
    return dict(window=hex(window), title=title, root_x=x.value, root_y=y.value, width=width.value, height=height.value)

def walk(window):
    gr, parent, children, count = W(), W(), C.POINTER(W)(), C.c_uint()
    if X.XQueryTree(display, window, C.byref(gr), C.byref(parent), C.byref(children), C.byref(count)):
        descendants = [children[index] for index in range(count.value)]
        if children:
            X.XFree(children)
        for child in descendants:
            item = inspect(child)
            if item['title']:
                print(json.dumps(item))
            walk(child)

try:
    if args.command == 'list':
        walk(root)
    else:
        if args.window is None:
            parser.error('--window is required')
        geometry = inspect(args.window)
        if args.command == 'inspect':
            print(json.dumps(geometry))
        elif args.command == 'close':
            event = Event()
            event.client.type = 33
            event.client.display = display
            event.client.window = args.window
            event.client.message_type = X.XInternAtom(display, b'WM_PROTOCOLS', 0)
            event.client.format = 32
            event.client.data.l[0] = X.XInternAtom(display, b'WM_DELETE_WINDOW', 0)
            event.client.data.l[1] = 0
            if not X.XSendEvent(display, args.window, 0, 0, C.byref(event)):
                raise RuntimeError('WM_DELETE_WINDOW delivery failed')
        else:
            X.XRaiseWindow(display, args.window)
            X.XSetInputFocus(display, args.window, 2, 0)
            X.XSync(display, 0)
            if args.command in ('move', 'click'):
                if args.x is None or args.y is None:
                    parser.error('--x/--y client coordinates are required')
                if not (0 <= args.x < geometry['width'] and 0 <= args.y < geometry['height']):
                    parser.error('client coordinates outside target window')
                T.XTestFakeMotionEvent(display, -1, geometry['root_x'] + args.x, geometry['root_y'] + args.y, 0)
                X.XSync(display, 0)
                time.sleep(0.1)
                if args.command == 'click':
                    T.XTestFakeButtonEvent(display, args.button, 1, 0)
                    X.XSync(display, 0)
                    time.sleep(args.hold)
                    T.XTestFakeButtonEvent(display, args.button, 0, 0)
            if args.command == 'key':
                keysym = X.XStringToKeysym(args.key.encode())
                keycode = X.XKeysymToKeycode(display, keysym)
                if not keycode:
                    raise RuntimeError('Unknown or unmapped keysym: ' + args.key)
                T.XTestFakeKeyEvent(display, keycode, 1, 0)
                X.XSync(display, 0)
                time.sleep(args.hold)
                T.XTestFakeKeyEvent(display, keycode, 0, 0)
        X.XSync(display, 0)
        if args.command != 'inspect':
            print(json.dumps(dict(command=args.command, target=geometry, key=args.key if args.command == 'key' else None, x=args.x, y=args.y)))
finally:
    X.XCloseDisplay(display)
