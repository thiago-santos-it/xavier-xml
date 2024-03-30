/*
From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html

** The DOM Structure Model**

The DOM presents documents as a hierarchy of Node objects that also implement other, more specialized interfaces. Some types of nodes may have child nodes of various types, and others are leaf nodes that cannot have anything below them in the document structure. The node types, and which node types they may have as children, are as follows:

Document -- Element (maximum of one), ProcessingInstruction, Comment, DocumentType
DocumentFragment -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
DocumentType -- no children
EntityReference -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
Element -- Element, Text, Comment, ProcessingInstruction, CDATASection, EntityReference
Attr -- Text, EntityReference
ProcessingInstruction -- no children
Comment -- no children
Text -- no children
CDATASection -- no children
Entity -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
Notation -- no children
The DOM also specifies a NodeList interface to handle ordered lists of Nodes, such as the children of a Node, or the elements returned by the Element.getElementsByTagName method, and also a NamedNodeMap interface to handle unordered sets of nodes referenced by their name attribute, such as the attributes of an Element. NodeLists and NamedNodeMaps in the DOM are "live", that is, changes to the underlying document structure are reflected in all relevant NodeLists and NamedNodeMaps. For example, if a DOM user gets a NodeList object containing the children of an Element, then subsequently adds more children to that element (or removes children, or modifies them), those changes are automatically reflected in the NodeList without further action on the user's part. Likewise changes to a Node in the tree are reflected in all references to that Node in NodeLists and NamedNodeMaps.
 */

pub mod attr;
pub mod character_data;
pub mod comment;
pub mod doc_type;
pub mod document;
pub mod element;
pub mod entity;
pub mod entity_ref;
pub mod fragment;
pub mod implementation;
pub mod instruction;
pub mod named_node_map;
pub mod node_list;
pub mod notation;
pub mod text;
pub mod error;