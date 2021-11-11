use byteorder::{LittleEndian, ReadBytesExt};
use libc::c_void;
use std::{
    fs::File,
    io::{self, Read},
};

type DtTileRef = u64;
type DtPolyRef = u64;
type DtStatus = u32;
type DtNavMesh = c_void;
type DtNavMeshQuery = c_void;
type DtQueryFilter = c_void;

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

#[repr(C)]
#[derive(Debug)]
pub struct MmapTileHeader {
    magic: u32,
    dt_version: u32,
    mmap_version: u32,
    size: u32,
    use_liquids: u32,
}

impl MmapTileHeader {
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let magic = rdr.read_u32::<LittleEndian>()?;
        let dt_version = rdr.read_u32::<LittleEndian>()?;
        let mmap_version = rdr.read_u32::<LittleEndian>()?;

        let size = rdr.read_u32::<LittleEndian>()?;
        let use_liquids = rdr.read_u32::<LittleEndian>()?;

        Ok(MmapTileHeader {
            magic,
            dt_version,
            mmap_version,
            size,
            use_liquids,
        })
    }
}

#[link(name = "detour", kind = "static")]
extern "C" {
    pub fn dtNavMesh_alloc() -> *mut DtNavMesh;
    pub fn dtNavMesh_init(_self: *mut DtNavMesh, params: *const DtNavMeshParams) -> DtStatus;
    pub fn dtNavMesh_initSingle(
        _self: *mut DtNavMesh,
        data: *mut u8,
        dataSize: i32,
        flags: i32,
    ) -> DtStatus;
    pub fn dtNavMesh_addTile(
        _self: *mut DtNavMesh,
        data: *mut u8,
        dataSize: i32,
        flags: i32,
        lastRef: DtTileRef,
        result: *mut DtTileRef,
    ) -> DtStatus;

    pub fn dtQueryFilter_alloc() -> *mut DtQueryFilter;
    pub fn dtQueryFilter_setIncludeFlags(_self: *mut DtQueryFilter, include_flags: u16);
    pub fn dtQueryFilter_getIncludeFlags(_self: *mut DtQueryFilter) -> u16;
    pub fn dtQueryFilter_setExcludeFlags(_self: *mut DtQueryFilter, exclude_flags: u16);

    pub fn dtNavMeshQuery_alloc() -> *mut DtNavMeshQuery;
    pub fn dtNavMeshQuery_init(
        _self: *mut DtNavMeshQuery,
        dtNavMesh: *mut DtNavMesh,
        max_nodes: i32,
    ) -> DtStatus;
    pub fn dtNavMeshQuery_findNearestPoly(
        _self: *mut DtNavMeshQuery,
        center: *const f32,
        extents: *const f32,
        filter: *const DtQueryFilter,
        nearestRef: *mut DtPolyRef,
        nearestPt: *mut f32,
    ) -> DtStatus;
}

fn add_tile(nav_mesh: *mut DtNavMesh, file_name: &str) -> DtStatus {
    println!("Loading {:#?}", file_name);
    let mut tile_file = File::open(file_name).unwrap();
    let mmap_header = MmapTileHeader::from_reader(&mut tile_file).unwrap();
    println!("{:#?}", mmap_header);

    let mut vec_buffer = Vec::with_capacity(mmap_header.size.try_into().unwrap());
    tile_file.read_to_end(&mut vec_buffer).unwrap();

    let buffer = unsafe { libc::malloc(mmap_header.size.try_into().unwrap()) as *mut u8 };
    unsafe {
        std::ptr::copy_nonoverlapping(vec_buffer.as_ptr(), buffer, vec_buffer.len());
        let tile_ref = 0u64;
        dtNavMesh_addTile(
            nav_mesh,
            buffer,
            mmap_header.size.try_into().unwrap(),
            1,
            0,
            &tile_ref as *const _ as *mut u64,
        )
    }
}

fn main() -> io::Result<()> {
    let nav_mesh = unsafe { dtNavMesh_alloc() };
    let map_params_file = File::open("./mmaps/530.mmap")?;
    let params = DtNavMeshParams::from_reader(map_params_file)?;
    println!("Params: {:#?}", params);

    let navmesh_init_status =
        unsafe { dtNavMesh_init(nav_mesh, &params as *const DtNavMeshParams) };
    println!("Nav Mesh Init: {:?}", navmesh_init_status);

    let nav_mesh_query = unsafe { dtNavMeshQuery_alloc() };
    let nav_mesh_query_status = unsafe { dtNavMeshQuery_init(nav_mesh_query, nav_mesh, 1024) };
    println!("Nav Mesh Query Status: {:?}", nav_mesh_query_status);

    //let tiles = ["1543"];
    let tiles = ["1543", "3418", "3518"];
    for tile in tiles {
        let load_status = add_tile(nav_mesh, &["./mmaps/530", tile, ".mmtile"].join(""));
        println!("Load Result: {:?}", load_status);
    }

    let filter = unsafe { dtQueryFilter_alloc() };
    let include_flags: u16 = 1 | 8 | 4 | 2;
    let exclude_flags: u16 = 0;

    unsafe {
        dtQueryFilter_setIncludeFlags(filter, include_flags);
        dtQueryFilter_setExcludeFlags(filter, exclude_flags);

        let raw_start = [8850.49, -6091.11, -1.52];
        let start = [raw_start[1], raw_start[2], raw_start[0]];
        let extents = [3.0f32, 5.0f32, 3.0f32];
        let mut closest_point = [0f32, 0f32, 0f32];

        let nearest_ref: DtPolyRef = 0;
        let nearest_status = dtNavMeshQuery_findNearestPoly(
            nav_mesh_query,
            start.as_ptr(),
            extents.as_ptr(),
            filter as *const _ as *const DtQueryFilter,
            &nearest_ref as *const _ as *mut u64,
            closest_point.as_mut_ptr(),
        );

        println!("Nearest Status: {:?}", nearest_status);
        println!("Nearest Ref: {:?}", nearest_ref);
        println!("Closest Point: {:#?}", closest_point);
    }

    Ok(())
}
