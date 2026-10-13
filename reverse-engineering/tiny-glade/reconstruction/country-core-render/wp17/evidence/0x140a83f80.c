// 0x140a83f80 ev_height_changed

void __cdecl country_core::resources::history::wall_history::impl$0::ev_height_changed(void)

{
  EditType::new_height_changed();
  History::add();
  return;
}

