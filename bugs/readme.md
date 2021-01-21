# Bugs

Similar to the progress folder, this folder documents interesting or entertaining bugs found during the development of the project.

## Index

|                 Image                 |       Date       |    Description                                                                      |
|---------------------------------------|------------------|-------------------------------------------------------------------------------------|
|![001_ring.png](./001_ring.png)        | 2021-01-08T01:51 | A dark ring appears on the sphere, likely because of lack of visibility testing.
|![002_fp_error.png](./002_fp_error.png)| 2021-01-09T02:47 | In an attempt to remove the dark ring, I added visibility testing, but there's still floating point error!
|![003_cone.png](./003_cone.png)        | 2021-01-20T19:39 | Adjusting the camera position even slightly creates a weird code effect where the sphere dissapears.
|![004_position.png](./004_position.png)| 2021-01-21T00:13 | After fixing the cone issue above, I now can't position spheres in any way