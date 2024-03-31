/*
Interface EntityReference
--
EntityReference objects may be inserted into the structure model when an entity reference is in the
source document, or when the user wishes to insert an entity reference. Note that character
references and references to predefined entities are considered to be expanded by the HTML or XML
processor so that characters are represented by their Unicode equivalent rather than by an entity
reference. Moreover, the XML processor may completely expand references to entities while building
the structure model, instead of providing EntityReference objects. If it does provide such objects,
then for a given EntityReference node, it may be that there is no Entity node representing the
referenced entity; but if such an Entity exists, then the child list of the EntityReference node is
the same as that of the Entity node. As with the Entity node, all descendants of the EntityReference
are readonly.

The resolution of the children of the EntityReference (the replacement value of the referenced
Entity) may be lazily evaluated; actions by the user (such as calling the childNodes method on the
EntityReference node) are assumed to trigger the evaluation.

IDL Definition
interface EntityReference : Node {
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */
use crate::dom::node::{NodeImpl, NodeTrait};

pub struct EntityReference<'a> {
    /*
    Inner state of node
     */
    inner: NodeImpl<'a>
}

impl NodeTrait for EntityReference<'_> {
    fn inner(&mut self) -> &mut NodeImpl {
        //&mut self.inner
        unimplemented!()
    }
}
