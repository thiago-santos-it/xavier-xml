/*
Interface DocumentType
--
Each Document has a doctype attribute whose value is either null or a DocumentType object. The DocumentType interface in the DOM Level 1 Core provides an interface to the list of entities that are defined for the document, and little else because the effect of namespaces and the various XML scheme efforts on DTD representation are not clearly understood as of this writing.

The DOM Level 1 doesn't support editing DocumentType nodes.

IDL Definition
interface DocumentType : Node {
  readonly attribute  DOMString            name;
  readonly attribute  NamedNodeMap         entities;
  readonly attribute  NamedNodeMap         notations;
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
*/

use crate::dom::named_node_map::NamedNodeMap;
use crate::dom::node::{NodeImpl, NodeTrait};
use crate::dom::string::DOMString;

pub struct DocumentType<'a> {
    /*
    The name of DTD; i.e., the name immediately following the DOCTYPE keyword.
    */
    pub name: DOMString,
    /*
    A NamedNodeMap containing the general entities, both external and internal, declared in the DTD.
    Duplicates are discarded. For example in:

    ```xml <!DOCTYPE ex SYSTEM "ex.dtd" [
      <!ENTITY foo "foo">
      <!ENTITY bar "bar">
      <!ENTITY % baz "baz">
    ]>
    <ex/>```

    The interface provides access to foo and bar but not baz. Every node in this map also implements
    the Entity interface.

    The DOM Level 1 does not support editing entities, therefore entities cannot be altered in any way.
     */
    pub entities: NamedNodeMap,
    /*
    A NamedNodeMap containing the notations declared in the DTD. Duplicates are discarded. Every
    node in this map also implements the Notation interface.

    The DOM Level 1 does not support editing notations, therefore notations cannot be altered in
    any way.
     */
    pub notations: NamedNodeMap,
    /*
    Inner state of node
     */
    inner: NodeImpl<'a>
}

impl NodeTrait for DocumentType<'_> {
    fn inner(&mut self) -> &mut NodeImpl {
        //&mut self.inner
        unimplemented!()
    }
}
