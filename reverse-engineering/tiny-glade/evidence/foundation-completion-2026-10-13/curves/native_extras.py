"""Additional concrete B specializations, with explicit input/output dimensions."""
import json, random, struct
import native_completion as q


def invoke(points, address, value, input_dimension=3, output_dimension=3, projected=False, integer=False):
    built = q.n.curve(points, input_dimension, projected)
    if 'error' in built:
        return {'input_error': built}
    q.cpu.mem_write(q.DATA + 0x400, bytes(q.cpu.mem_read(q.DATA, 56)))
    q.arguments(q.DATA + 0x800, q.DATA + 0x400)
    q.cpu.reg_write(q.UC_X86_REG_R8, value if integer else 0)
    if not integer:
        q.cpu.reg_write(q.UC_X86_REG_XMM2, q.bits(value))
    q.cpu.reg_write(q.UC_X86_REG_R9, 0)
    try:
        q.call(address)
        return q.decode_curve(q.DATA + 0x800, output_dimension)
    except RuntimeError as error:
        if 'original panic' not in str(error):
            raise
        return {'panic': True}


def main():
    rows = []
    shape = [[0, 0, 0], [3, 0, 4], [6, 0, 4]]
    for value in [0.0, -1.0, float('nan'), float('inf'), -float('inf')]:
        for name, address, projected in [('edge_global', 0x140152b00, False), ('edge_per_segment', 0x14146a2a0, True)]:
            rows.append({'method': name, 'points': [[q.bits(v) for v in p] for p in shape], 'argument': q.bits(value), 'expected': invoke(shape, address, value, projected=projected)})
    rng = random.Random(127042)
    for index in range(40):
        points = [[rng.uniform(-5, 5), rng.uniform(-5, 5)] for _ in range(rng.randint(2, 9))]
        if index % 3 == 0:
            points[-1] = points[0].copy()
        if points[-1] == points[0] and len(points) == 2:
            points.append([1, 1])
        for count in [2, 3, 5, 11]:
            rows.append({'method': 'approx_count', 'points': [[q.bits(v) for v in p] for p in points], 'argument': count, 'expected': invoke(points, 0x140153de0, count, 2, 2, integer=True)})
        for spacing in [0.7, 2, 10]:
            rows.append({'method': 'flat_remap_per_segment', 'points': [[q.bits(v) for v in p] for p in points], 'argument': q.bits(spacing), 'expected': invoke(points, 0x14146b5f0, spacing, 2, 3)})
    for _ in range(20):
        points = [[rng.uniform(-5, 5), rng.uniform(-5, 5)] for _ in range(rng.randint(2, 8))]
        other = [[rng.uniform(-5, 5), rng.uniform(-5, 5)] for _ in points]
        q.n.curve(points)
        q.cpu.mem_write(q.DATA + 0x5000, struct.pack('<' + 'f' * (len(other) * 2), *[v for p in other for v in p]))
        q.cpu.mem_write(q.DATA + 0x600, struct.pack('<QQQ', len(other), q.DATA + 0x5000, len(other)))
        q.arguments(q.DATA + 0x400, q.DATA + 0x600, 0)
        q.call(0x141469430)
        t = rng.uniform(-1.0, 2.0)
        q.arguments(q.DATA + 0x800, q.DATA, q.DATA + 0x400)
        q.cpu.reg_write(q.UC_X86_REG_XMM3, q.bits(t))
        q.cpu.mem_write(q.STACK + 0x100008 + 0x28, struct.pack('<Q', 0))
        q.call(0x141466a40)
        rows.append({'method': 'lerp', 'points': [[q.bits(v) for v in p] for p in points], 'other': [[q.bits(v) for v in p] for p in other], 'argument': q.bits(t), 'expected': q.decode_curve(q.DATA + 0x800, 2)})
    (q.OUT / 'extra-fixtures.json').write_text(json.dumps({'original_sha256': q.SHA, 'cases': rows}, indent=2) + '\n')
    print(json.dumps({'cases': len(rows)}))


if __name__ == '__main__':
    main()
