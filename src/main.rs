use byteorder::{LittleEndian, ReadBytesExt};
use libc::c_void;
use std::{
    fs::File,
    io::{self, Read},
};

type DtTileRef = u64;

#[repr(u32)]
#[derive(Debug)]
pub enum DtStatus {
    Failure = 1u32 << 31,
    Success = 1u32 << 30,
    InProgress = 1u32 << 29,
}

#[repr(C)]
#[derive(Debug)]
pub struct DtNavMeshParams {
    origin: [f32; 3],
    tile_width: f32,
    tile_height: f32,
    max_tiles: i32,
    max_polys: i32,
}

impl DtNavMeshParams {
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let origin_x = rdr.read_f32::<LittleEndian>()?;
        let origin_y = rdr.read_f32::<LittleEndian>()?;
        let origin_z = rdr.read_f32::<LittleEndian>()?;

        let tile_width = rdr.read_f32::<LittleEndian>()?;
        let tile_height = rdr.read_f32::<LittleEndian>()?;

        let max_tiles = rdr.read_i32::<LittleEndian>()?;
        let max_polys = rdr.read_i32::<LittleEndian>()?;

        Ok(DtNavMeshParams {
            origin: [origin_x, origin_y, origin_z],
            tile_width,
            tile_height,
            max_tiles,
            max_polys,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DtMeshHeader {
    magic: i32,
    version: i32,
    x: i32,
    y: i32,
    layer: i32,
    user_id: u32,
    poly_count: i32,
    vert_count: i32,
    max_link_count: i32,
    detail_mesh_count: i32,
    detail_vert_count: i32,
    detail_tri_count: i32,
    bv_node_count: i32,
    off_mesh_con_count: i32,
    off_mesh_base: i32,
    walkable_height: f32,
    walkable_climb: f32,
    b_min: [f32; 3],
    b_max: [f32; 3],
    bv_quant_factor: f32,
}

impl DtMeshHeader {
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let magic = rdr.read_i32::<LittleEndian>()?;
        let version = rdr.read_i32::<LittleEndian>()?;
        let x = rdr.read_i32::<LittleEndian>()?;
        let y = rdr.read_i32::<LittleEndian>()?;
        let layer = rdr.read_i32::<LittleEndian>()?;
        let user_id = rdr.read_u32::<LittleEndian>()?;
        let poly_count = rdr.read_i32::<LittleEndian>()?;
        let vert_count = rdr.read_i32::<LittleEndian>()?;
        let max_link_count = rdr.read_i32::<LittleEndian>()?;
        let detail_mesh_count = rdr.read_i32::<LittleEndian>()?;
        let detail_vert_count = rdr.read_i32::<LittleEndian>()?;
        let detail_tri_count = rdr.read_i32::<LittleEndian>()?;
        let bv_node_count = rdr.read_i32::<LittleEndian>()?;
        let off_mesh_con_count = rdr.read_i32::<LittleEndian>()?;
        let off_mesh_base = rdr.read_i32::<LittleEndian>()?;

        let walkable_height = rdr.read_f32::<LittleEndian>()?;
        let walkable_climb = rdr.read_f32::<LittleEndian>()?;

        let b_min_x = rdr.read_f32::<LittleEndian>()?;
        let b_min_y = rdr.read_f32::<LittleEndian>()?;
        let b_min_z = rdr.read_f32::<LittleEndian>()?;

        let b_max_x = rdr.read_f32::<LittleEndian>()?;
        let b_max_y = rdr.read_f32::<LittleEndian>()?;
        let b_max_z = rdr.read_f32::<LittleEndian>()?;

        let bv_quant_factor = rdr.read_f32::<LittleEndian>()?;

        Ok(DtMeshHeader {
            magic,
            version,
            x,
            y,
            layer,
            user_id,
            poly_count,
            vert_count,
            max_link_count,
            detail_mesh_count,
            detail_vert_count,
            detail_tri_count,
            bv_node_count,
            off_mesh_con_count,
            off_mesh_base,
            walkable_height,
            walkable_climb,
            b_min: [b_min_x, b_min_y, b_min_z],
            b_max: [b_max_x, b_max_y, b_max_z],
            bv_quant_factor,
        })
    }
}

#[link(name = "detour", kind = "static")]
extern "C" {
    pub fn dtNavMesh_alloc() -> *mut c_void;
    pub fn dtNavMesh_init(dtNavMesh: *mut c_void, params: *const DtNavMeshParams) -> DtStatus;
    pub fn dtNavMesh_getMaxTiles(dtNavMesh: *mut c_void) -> i32;
    pub fn dtNavMesh_addTile(
        dtNavMesh: *mut c_void,
        data: *mut u8,
        dataSize: i32,
        flags: i32,
        lastRef: DtTileRef,
        result: *mut DtTileRef,
    ) -> DtStatus;
}

fn main() {
    println!("Test");
    let nav_mesh = unsafe { dtNavMesh_alloc() };

    let map_params_file = File::open("530.mmap").unwrap();
    let params = DtNavMeshParams::from_reader(map_params_file).unwrap();
    println!("{:#?}", params);

    let status = unsafe { dtNavMesh_init(nav_mesh, &params) };
    println!("{:#?}", status);
}
