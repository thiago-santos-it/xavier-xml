
/*
Interface Notation
--
This interface represents a notation declared in the DTD. A notation either declares, by name, the format of an unparsed entity (see section 4.7 of the XML 1.0 specification), or is used for formal declaration of Processing Instruction targets (see section 2.6 of the XML 1.0 specification). The nodeName attribute inherited from Node is set to the declared name of the notation.

The DOM Level 1 does not support editing Notation nodes; they are therefore readonly.

A Notation node does not have any parent.

IDL Definition
interface Notation : Node {
  readonly attribute  DOMString            publicId;
  readonly attribute  DOMString            systemId;
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
*/

use crate::dom::node::{NodeImpl, NodeTrait};
use crate::dom::string::DOMString;

pub struct Notation<'a> {
    /*
    The public identifier of this notation. If the public identifier was not specified, this is null.
     */
    pub public_id: DOMString,
    /*
    The system identifier of this notation. If the system identifier was not specified, this is null.
     */
    pub system_id: DOMString,

    /*
    Inner state of node
     */
    inner: NodeImpl<'a>
}

impl NodeTrait for Notation<'_> {
    fn inner(&mut self) -> &mut NodeImpl {
        //&mut self.inner
        unimplemented!()
    }
}