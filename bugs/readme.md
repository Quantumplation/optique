# Bugs

Similar to the progress folder, this folder documents interesting or entertaining bugs found during the development of the project.

## Index

|                           Image                            |       Date       |    Description                                                                      |
|------------------------------------------------------------|------------------|-------------------------------------------------------------------------------------|
|<img src="./001_ring.png" width="100" height="100">         | 2021-01-08T01:51 | A dark ring appears on the sphere, likely because of lack of visibility testing.
|<img src="./002_fp_error.png" width="100" height="100">     | 2021-01-09T02:47 | In an attempt to remove the dark ring, I added visibility testing, but there's still floating point error!
|<img src="./003_cone.png" width="100" height="100">         | 2021-01-20T19:39 | Adjusting the camera position even slightly creates a weird code effect where the sphere disappears.
|<img src="./004_position.png" width="100" height="100">     | 2021-01-21T00:13 | After fixing the cone issue above, I now can't position spheres in any way
|<img src="./005_occlusion.png" width="100" height="100">    | 2021-01-21T00:13 | Occlusion testing is failing, meaning I'm not casting shadows properly.  The green is intentional, all points that are occluded.
|<img src="./006_lambert.png" width="100" height="100">      | 2021-01-25T21:22 | While implementing fresnel/reflection models, lambertian reflection gives an odd result.
|<img src="./007_singularities.png" width="100" height="100">| 2021-01-26T00:37 | There are odd singularities in sphere normals
|<img src="./008_triangle.png" width="100" height="100">     | 2021-01-27T03:33 | There are odd collision testing issues with triangles.  Likely FP error for the occlusion ray
|<img src="./009_bvh.png" width="100" height="100">          | 2021-01-28T04:11 | This weird eldritch horror came up when implementing bounding-volume-hierarchy aggregators