// call_once<country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure_env$0,tuple$<>_>  size=85
// void __cdecl call_once<country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure_env$0,tuple$<>_>(void)

void __cdecl
core::ops::function::FnOnce::
call_once<country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure_0::closure_env_0,tuple_<>_>
          (void)

{
  undefined8 *in_RCX;
  undefined8 local_40;
  undefined8 uStack_38;
  undefined8 local_30;
  longlong local_28;
  
  ::utils::geometry::circle::Circle2d::as_curve3();
  in_RCX[2] = local_30;
  *in_RCX = local_40;
  in_RCX[1] = uStack_38;
  if (local_28 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}



// CALLEES: [{'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'as_curve3', 'address': '140c97e10'}]