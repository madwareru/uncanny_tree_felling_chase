pub fn system(world: &mut hecs::World) {
    for (_, data) in world.query_mut::<(&mut crate::types::GlobalStorage,)>() {
        let global_storage: &mut crate::types::GlobalStorage = data.0;
        global_storage.scene_compositor.flush_drawing_queue(
            global_storage.atlas_definition.tile_width * global_storage.tilemap.w,
            global_storage.atlas_definition.tile_height * global_storage.tilemap.h
        );
    }
}