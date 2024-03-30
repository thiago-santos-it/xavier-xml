/*
Interface Node
--
The Node interface is the primary datatype for the entire Document Object Model. It represents a single node in the document tree. While all objects implementing the Node interface expose methods for dealing with children, not all objects implementing the Node interface may have children. For example, Text nodes may not have children, and adding children to such nodes results in a DOMException being raised.

The attributes nodeName, nodeValue and attributes are included as a mechanism to get at node information without casting down to the specific derived interface. In cases where there is no obvious mapping of these attributes for a specific nodeType (e.g., nodeValue for an Element or attributes for a Comment), this returns null. Note that the specialized interfaces may contain additional and more convenient mechanisms to get and set the relevant information.

IDL Definition
interface Node {
  // NodeType
  const unsigned short      ELEMENT_NODE       = 1;
  const unsigned short      ATTRIBUTE_NODE     = 2;
  const unsigned short      TEXT_NODE          = 3;
  const unsigned short      CDATA_SECTION_NODE = 4;
  const unsigned short      ENTITY_REFERENCE_NODE = 5;
  const unsigned short      ENTITY_NODE        = 6;
  const unsigned short      PROCESSING_INSTRUCTION_NODE = 7;
  const unsigned short      COMMENT_NODE       = 8;
  const unsigned short      DOCUMENT_NODE      = 9;
  const unsigned short      DOCUMENT_TYPE_NODE = 10;
  const unsigned short      DOCUMENT_FRAGMENT_NODE = 11;
  const unsigned short      NOTATION_NODE      = 12;

  readonly attribute  DOMString            nodeName;
           attribute  DOMString            nodeValue;
                                                 // raises(DOMException) on setting
                                                 // raises(DOMException) on retrieval
  readonly attribute  unsigned short       nodeType;
  readonly attribute  Node                 parentNode;
  readonly attribute  NodeList             childNodes;
  readonly attribute  Node                 firstChild;
  readonly attribute  Node                 lastChild;
  readonly attribute  Node                 previousSibling;
  readonly attribute  Node                 nextSibling;
  readonly attribute  NamedNodeMap         attributes;
  readonly attribute  Document             ownerDocument;
  Node                      insertBefore(in Node newChild,
                                         in Node refChild)
                                         raises(DOMException);
  Node                      replaceChild(in Node newChild,
                                         in Node oldChild)
                                         raises(DOMException);
  Node                      removeChild(in Node oldChild)
                                        raises(DOMException);
  Node                      appendChild(in Node newChild)
                                        raises(DOMException);
  boolean                   hasChildNodes();
  Node                      cloneNode(in boolean deep);
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html

Definition group NodeType
An integer indicating which type of node this is.

Defined Constants
ELEMENT_NODE
The node is a Element.

ATTRIBUTE_NODE
The node is an Attr.

TEXT_NODE
The node is a Text node.

CDATA_SECTION_NODE
The node is a CDATASection.

ENTITY_REFERENCE_NODE
The node is an EntityReference.

ENTITY_NODE
The node is an Entity.

PROCESSING_INSTRUCTION_NODE
The node is a ProcessingInstruction.

COMMENT_NODE
The node is a Comment.

DOCUMENT_NODE
The node is a Document.

DOCUMENT_TYPE_NODE
The node is a DocumentType.

DOCUMENT_FRAGMENT_NODE
The node is a DocumentFragment.

NOTATION_NODE
The node is a Notation.

The values of nodeName, nodeValue, and attributes vary according to the node type as follows:
nodeName	nodeValue	attributes
Element	tagName	null	NamedNodeMap
Attr	name of attribute	value of attribute	null
Text	#text	content of the text node	null
CDATASection	#cdata-section	content of the CDATA Section	null
EntityReference	name of entity referenced	null	null
Entity	entity name	null	null
ProcessingInstruction	target	entire content excluding the target	null
Comment	#comment	content of the comment	null
Document	#document	null	null
DocumentType	document type name	null	null
DocumentFragment	#document-fragment	null	null
Notation	notation name	null	null
Attributes
nodeName
The name of this node, depending on its type; see the table above.
nodeValue
The value of this node, depending on its type; see the table above.
Exceptions on setting
DOMException
NO_MODIFICATION_ALLOWED_ERR: Raised when the node is readonly.

Exceptions on retrieval
DOMException
DOMSTRING_SIZE_ERR: Raised when it would return more characters than fit in a DOMString variable on the implementation platform.

nodeType
A code representing the type of the underlying object, as defined above.
parentNode
The parent of this node. All nodes, except Document, DocumentFragment, and Attr may have a parent. However, if a node has just been created and not yet added to the tree, or if it has been removed from the tree, this is null.
childNodes
A NodeList that contains all children of this node. If there are no children, this is a NodeList containing no nodes. The content of the returned NodeList is "live" in the sense that, for instance, changes to the children of the node object that it was created from are immediately reflected in the nodes returned by the NodeList accessors; it is not a static snapshot of the content of the node. This is true for every NodeList, including the ones returned by the getElementsByTagName method.
firstChild
The first child of this node. If there is no such node, this returns null.
lastChild
The last child of this node. If there is no such node, this returns null.
previousSibling
The node immediately preceding this node. If there is no such node, this returns null.
nextSibling
The node immediately following this node. If there is no such node, this returns null.
attributes
A NamedNodeMap containing the attributes of this node (if it is an Element) or null otherwise.
ownerDocument
The Document object associated with this node. This is also the Document object used to create new nodes. When this node is a Document this is null.
Methods
insertBefore
Inserts the node newChild before the existing child node refChild. If refChild is null, insert newChild at the end of the list of children.
If newChild is a DocumentFragment object, all of its children are inserted, in the same order, before refChild. If the newChild is already in the tree, it is first removed.

Parameters
newChild
The node to insert.

refChild
The reference node, i.e., the node before which the new node must be inserted.

Return Value
The node being inserted.
Exceptions
DOMException
HIERARCHY_REQUEST_ERR: Raised if this node is of a type that does not allow children of the type of the newChild node, or if the node to insert is one of this node's ancestors.

WRONG_DOCUMENT_ERR: Raised if newChild was created from a different document than the one that created this node.

NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

NOT_FOUND_ERR: Raised if refChild is not a child of this node.


replaceChild
Replaces the child node oldChild with newChild in the list of children, and returns the oldChild node. If the newChild is already in the tree, it is first removed.
Parameters
newChild
The new node to put in the child list.

oldChild
The node being replaced in the list.

Return Value
The node replaced.
Exceptions
DOMException
HIERARCHY_REQUEST_ERR: Raised if this node is of a type that does not allow children of the type of the newChild node, or it the node to put in is one of this node's ancestors.

WRONG_DOCUMENT_ERR: Raised if newChild was created from a different document than the one that created this node.

NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

NOT_FOUND_ERR: Raised if oldChild is not a child of this node.


removeChild
Removes the child node indicated by oldChild from the list of children, and returns it.
Parameters
oldChild
The node being removed.

Return Value
The node removed.
Exceptions
DOMException
NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

NOT_FOUND_ERR: Raised if oldChild is not a child of this node.


appendChild
Adds the node newChild to the end of the list of children of this node. If the newChild is already in the tree, it is first removed.
Parameters
newChild
The node to add.

If it is a DocumentFragment object, the entire contents of the document fragment are moved into the child list of this node

Return Value
The node added.
Exceptions
DOMException
HIERARCHY_REQUEST_ERR: Raised if this node is of a type that does not allow children of the type of the newChild node, or if the node to append is one of this node's ancestors.

WRONG_DOCUMENT_ERR: Raised if newChild was created from a different document than the one that created this node.

NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.


hasChildNodes
This is a convenience method to allow easy determination of whether a node has any children.
Return Value
true if the node has any children, false if the node has no children.

This method has no parameters.
This method raises no exceptions.
cloneNode
Returns a duplicate of this node, i.e., serves as a generic copy constructor for nodes. The duplicate node has no parent (parentNode returns null.).
Cloning an Element copies all attributes and their values, including those generated by the XML processor to represent defaulted attributes, but this method does not copy any text it contains unless it is a deep clone, since the text is contained in a child Text node. Cloning any other type of node simply returns a copy of this node.

Parameters
deep
If true, recursively clone the subtree under the specified node; if false, clone only the node itself (and its attributes, if it is an Element).

Return Value
The duplicate node.

This method raises no exceptions.
Interface NodeList
The NodeList interface provides the abstraction of an ordered collection of nodes, without defining or constraining how this collection is implemented.

The items in the NodeList are accessible via an integral index, starting from 0.

IDL Definition
interface NodeList {
  Node                      item(in unsigned long index);
  readonly attribute  unsigned long        length;
};

Methods
item
Returns the indexth item in the collection. If index is greater than or equal to the number of nodes in the list, this returns null.
Parameters
index
Index into the collection.

Return Value
The node at the indexth position in the NodeList, or null if that is not a valid index.

This method raises no exceptions.
Attributes
length
The number of nodes in the list. The range of valid child node indices is 0 to length-1 inclusive.
 */

enum Node {

}