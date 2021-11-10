#include "DetourStatus.h"
#include "DetourNavMesh.h"

extern "C"
{

  dtNavMesh *dtNavMesh_alloc()
  {
    return dtAllocNavMesh();
  }

  dtStatus dtNavMesh_init(dtNavMesh *navMesh, const dtNavMeshParams *params)
  {
    return navMesh->init(params);
  }

  int dtNavMesh_getMaxTiles(dtNavMesh *navMesh)
  {
    return navMesh->getMaxTiles();
  }

  dtStatus dtNavMesh_addTile(dtNavMesh *navMesh, unsigned char *data, int dataSize, int flags, dtTileRef lastRef, dtTileRef *result)
  {
    return navMesh->addTile(data, dataSize, flags, lastRef, result);
  }
};