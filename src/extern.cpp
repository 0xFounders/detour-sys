#include "DetourStatus.h"
#include "DetourNavMesh.h"
#include "DetourNavMeshQuery.h"
#include <iostream>

extern "C"
{

  dtNavMesh *dtNavMesh_alloc()
  {
    return dtAllocNavMesh();
  }

  dtStatus dtNavMesh_init(dtNavMesh *mesh, const dtNavMeshParams *params)
  {
    return mesh->init(params);
  }

  dtStatus dtNavMesh_initSingle(dtNavMesh *mesh, unsigned char *data, int dataSize, int flags)
  {
    return mesh->init(data, dataSize, flags);
  }

  dtStatus dtNavMesh_addTile(dtNavMesh *mesh, unsigned char *data, int dataSize,
                             int flags, dtTileRef lastRef, dtTileRef *result)
  {
    return mesh->addTile(data, dataSize, flags, lastRef, result);
  }

  dtNavMeshQuery *dtNavMeshQuery_alloc()
  {
    return dtAllocNavMeshQuery();
  }

  dtQueryFilter *dtQueryFilter_alloc()
  {
    return new dtQueryFilter();
  }

  void dtQueryFilter_setIncludeFlags(dtQueryFilter *filter, unsigned short flags)
  {
    filter->setIncludeFlags(flags);
  }

  unsigned short dtQueryFilter_getIncludeFlags(dtQueryFilter *filter)
  {
    return filter->getIncludeFlags();
  }

  void dtQueryFilter_setExcludeFlags(dtQueryFilter *filter, unsigned short flags)
  {
    filter->setExcludeFlags(flags);
  }

  dtStatus dtNavMeshQuery_init(dtNavMeshQuery *query, dtNavMesh *mesh, int maxNodes)
  {
    return query->init(mesh, maxNodes);
  }

  dtStatus dtNavMeshQuery_findNearestPoly(dtNavMeshQuery *query, const float *center, const float *extents,
                                          const dtQueryFilter *filter,
                                          dtPolyRef *nearestRef, float *nearestPt)
  {
    return query->findNearestPoly(center, extents, filter, nearestRef, nearestPt);
  }
};