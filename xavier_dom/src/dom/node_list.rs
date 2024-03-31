/*
Interface NodeList
--

The NodeList interface provides the abstraction of an ordered collection of nodes, without defining or constraining how this collection is implemented.

The items in the NodeList are accessible via an integral index, starting from 0.

IDL Definition
interface NodeList {
  Node                      item(in unsigned long index);
  readonly attribute  unsigned long        length;
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */

use crate::dom::node::Node;

pub struct NodeList {

    /*
    The number of nodes in the list. The range of valid child node indices is 0 to length-1 inclusive.
     */
    pub length: i32
}

impl NodeList {

    /*
    Returns the indexth item in the collection. If index is greater than or equal to the number of nodes in the list, this returns null.
    Parameters:
    - index: Index into the collection.

    Return Value:
    The node at the indexth position in the NodeList, or null if that is not a valid index.

    This method raises no exceptions.
     */
    pub fn item(&self, index: i32) -> Node {
        unimplemented!()
    }
}