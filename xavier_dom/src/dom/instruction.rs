/*
Interface ProcessingInstruction
--
The ProcessingInstruction interface represents a "processing instruction", used in XML as a way to
keep processor-specific information in the text of the document.

IDL Definition
interface ProcessingInstruction : Node {
  readonly attribute  DOMString            target;
           attribute  DOMString            data;
                                      // raises(DOMException) on setting
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */

use crate::dom::node::{NodeImpl, NodeTrait};
use crate::dom::string::DOMString;

pub struct ProcessingInstruction<'a> {
    /*
    The target of this processing instruction. XML defines this as being the first token following
    the markup that begins the processing instruction.
     */
    pub target: DOMString,
    /*
    The content of this processing instruction. This is from the first non white space character
    after the target to the character immediately preceding the ?>.

    Exceptions on setting
    DOMException
    - NO_MODIFICATION_ALLOWED_ERR: Raised when the node is readonly.
     */
    pub data: DOMString,
    /*
    Inner state of node
     */
    inner: NodeImpl<'a>
}

impl NodeTrait for ProcessingInstruction<'_> {
    fn inner(&mut self) -> &mut NodeImpl {
        //&mut self.inner
        unimplemented!()
    }
}
