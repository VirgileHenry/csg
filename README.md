# Csg objects abstraction

This library offers an abstraction over csg objects - a tree of operations and primitives. I've build it primarly to abstract away the csg part from my raymarcher [Morpheus](https://github.com/VirgileHenry/Morpheus). It is made to be flexible and offers a variety of features.

### Core

The core principle is that a csg object is either a operation between csg objects, or a primitive. This is reflected through the `csg::object::Object` enum, that can take ether of those values. 

However, I've been more flexible than usual: operations can be between any number of objects and not only two, as we often see. This was made as one of my target project is a 3D csg modeling tool, and I want the user to not be bothered by being required to give every operations exactly 2 nodes. This imply that I have a variant, the `csg::binary_oject::BinObject` that is a csg objects where operations are all `csg::bin_operations::BinOp`, meaning they all have exactly two child. The csg objects offer ways to convert them to binary objects.

I've also added modifiers, which are nodes with one child. They allow operations such as rounding, mirror, repeat, etc.

In the end, the `csg::object::Object` contains a bit more than only two possibilities, and this library offers definitively more than the minimal csg operations.

### Ids

- this is under construction -

The Id is a feature that will assign a unique id to each node of a given csg tree. This allow to later reference this node with this id, allowing the user to optimize their usage of this library.

### Carried data, user defined objects

- this is under construction -

This library aims to allow every node of the csg object to carry user data. This is again, a need for my own uses. If this is not necessary, the default csg objects will carry the type `()` (e.g. no data, and zero additional cost). Further more, this library allow the user to extend the possible set of csg objects, by creating custom ones as long as they implement the  required traits.