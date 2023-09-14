EGA
===

WIP implementation of a 3D PGA library for computer graphics.

---

"Projective Geometric Algebra" is a relatively small GA suitible for 3D
euclidean geometry, including rotations, translations, & reflections.

The algebra is 4 dimensional. A full-multivector contains 16 values; this makes
it comparable to the common 4x4 matrix from linear algebra. The implementation
here is simple & sparse, so many common products may be cheaper to compute
compared to a 4x4 matrix product. There are lots of unit tests, but there's
also likely a couple bugs & and lots of missing features.

Note: the products implemented largely follow the Cayley tables from
[bivector.net].

[bivector.net]: (https://bivector.net)

In the future I may try to implement the algebra detailed by
[rigidgeometricalgebra.org], which uses slightly different notation & products.

[rigidgeometricalgebra.org]: (https://rigidgeometricalgebra.org)

---

Some references I found helpful:

- L. Dorst and S. D. Keninck, “A Guided Tour to the Plane-Based Geometric Algebra PGA,” bivector.net, https://bivector.net/PGA4CS.pdf, 2022.
- C. G. Gunn, “Course notes Geometric Algebra for Computer Graphics SIGGRAPH 2019,” bivector.net, https://bivector.net/PROJECTIVE_GEOMETRIC_ALGEBRA.pdf, 2019.
- L. Dorst, S. Mann, and D. Fontijne, "Geometric Algebra for Computer Science: An Object-Oriented Approach to Geometry". Morgan Kaufmann Publishers, 2007.
- A. Macdonald, "Linear and Geometric Algebra". Charleston, SC: Alan Macdonald, 2017.
- E. Lengyel, "Foundations of Game Engine Development". Lincoln, CA: Terathon Software LLC, 2016.

License
-------

This crate is dual-licensed under either the
[Apache license, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
or the
[MIT license](http://opensource.org/licenses/MIT)
at your option.
