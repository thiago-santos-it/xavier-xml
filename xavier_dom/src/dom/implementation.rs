/*
Interface DOMImplementation
--

The DOMImplementation interface provides a number of methods for performing operations that are independent of any particular instance of the document object model.

The DOM Level 1 does not specify a way of creating a document instance, and hence document creation is an operation specific to an implementation. Future Levels of the DOM specification are expected to provide methods for creating documents directly.

IDL Definition
interface DOMImplementation {
  boolean                   hasFeature(in DOMString feature,
                                       in DOMString version);
};

Methods
hasFeature
Test if the DOM implementation implements a specific feature.
Parameters
feature
The package name of the feature to test. In Level 1, the legal values are "HTML" and "XML" (case-insensitive).

version
This is the version number of the package name to test. In Level 1, this is the string "1.0". If the version is not specified, supporting any version of the feature will cause the method to return true.

Return Value
true if the feature is implemented in the specified version, false otherwise.

This method raises no exceptions.
 */