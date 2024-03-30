/*
Interface Attr
--
The Attr interface represents an attribute in an Element object. Typically the allowable values for
the attribute are defined in a document type definition.

Attr objects inherit the Node interface, but since they are not actually child nodes of the element
they describe, the DOM does not consider them part of the document tree. Thus, the Node attributes
parentNode, previousSibling, and nextSibling have a null value for Attr objects. The DOM takes the
view that attributes are properties of elements rather than having a separate identity from the
elements they are associated with; this should make it more efficient to implement such features as
default attributes associated with all elements of a given type. Furthermore, Attr nodes may not be
immediate children of a DocumentFragment. However, they can be associated with Element nodes
contained within a DocumentFragment. In short, users and implementors of the DOM need to be aware
that Attr nodes have some things in common with other objects inheriting the Node interface, but
they also are quite distinct.

The attribute's effective value is determined as follows: if this attribute has been explicitly
assigned any value, that value is the attribute's effective value; otherwise, if there is a
declaration for this attribute, and that declaration includes a default value, then that default
value is the attribute's effective value; otherwise, the attribute does not exist on this element
in the structure model until it has been explicitly added. Note that the nodeValue attribute on the
Attr instance can also be used to retrieve the string version of the attribute's value(s).

In XML, where the value of an attribute can contain entity references, the child nodes of the Attr
node provide a representation in which entity references are not expanded. These child nodes may be
either Text or EntityReference nodes. Because the attribute type may be unknown, there are no
tokenized attribute values.

IDL Definition
interface Attr : Node {
  readonly attribute  DOMString            name;
  readonly attribute  boolean              specified;
           attribute  DOMString            value;
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */
pub struct Attr {
    /*Returns the name of this attribute.*/
    pub name: DOMString,
    /*
    If this attribute was explicitly given a value in the original document, this is true;
    otherwise, it is false. Note that the implementation is in charge of this attribute, not the
    user.
    If the user changes the value of the attribute (even if it ends up having the same value as the
    default value) then the specified flag is automatically flipped to true. To re-specify the
    attribute as the default value from the DTD, the user must delete the attribute. The
    implementation will then make a new attribute available with specified set to false and the
    default value (if one exists).

    In summary:

    - If the attribute has an assigned value in the document then specified is true, and the value
    is the assigned value.

    - If the attribute has no assigned value in the document and has a default value in the DTD,
    then specified is false, and the value is the default value in the DTD.

    - If the attribute has no assigned value in the document and has a value of #IMPLIED in the DTD,
    then the attribute does not appear in the structure model of the document.
     */
    pub specified: bool,
    /*
    On retrieval, the value of the attribute is returned as a string. Character and general entity
    references are replaced with their values.

    On setting, this creates a Text node with the unparsed contents of the string.
     */
    pub value: DOMString
}