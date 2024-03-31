/*
Interface NamedNodeMap
--
Objects implementing the NamedNodeMap interface are used to represent collections of nodes that can
be accessed by name. Note that NamedNodeMap does not inherit from NodeList; NamedNodeMaps are not
maintained in any particular order. Objects contained in an object implementing NamedNodeMap may
also be accessed by an ordinal index, but this is simply to allow convenient enumeration of the
contents of a NamedNodeMap, and does not imply that the DOM specifies an order to these Nodes.

IDL Definition
interface NamedNodeMap {
  Node                      getNamedItem(in DOMString name);
  Node                      setNamedItem(in Node arg)
                                         raises(DOMException);
  Node                      removeNamedItem(in DOMString name)
                                            raises(DOMException);
  Node                      item(in unsigned long index);
  readonly attribute  unsigned long        length;
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */

pub struct NamedNodeMap {
    /*
    The number of nodes in the map. The range of valid child node indices is 0 to length-1 inclusive.
     */
    pub length: i32
}


impl NamedNodeMap {

    /*
    Retrieves a node specified by name.

    Parameters:
    - name: Name of a node to retrieve.

    Return Value:
    A Node (of any type) with the specified name, or null if the specified name did not identify
    any node in the map.

    This method raises no exceptions.
     */
    pub fn get_named_item(name: DOMString) -> Node {
        unimplemented!();
    }

    /*
    Adds a node using its nodeName attribute.
    As the nodeName attribute is used to derive the name which the node must be stored under,
    multiple nodes of certain types (those that have a "special" string value) cannot be stored as
    the names would clash. This is seen as preferable to allowing nodes to be aliased.

    Parameters
    - arg: A node to store in a named node map. The node will later be accessible using the value of
    the nodeName attribute of the node. If a node with that name is already present in the map, it
    is replaced by the new one.

    Return Value:
    If the new Node replaces an existing node with the same name the previously existing Node is
    returned, otherwise null is returned.

    DOMException:
    - WRONG_DOCUMENT_ERR: Raised if arg was created from a different document than the one that
    created the NamedNodeMap.
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this NamedNodeMap is readonly.
    - INUSE_ATTRIBUTE_ERR: Raised if arg is an Attr that is already an attribute of another Element
    object. The DOM user must explicitly clone Attr nodes to re-use them in other elements.
     */
    pub fn set_named_item(arg: None) -> Node {
        unimplemented!();
    }

    /*
    Removes a node specified by name. If the removed node is an Attr with a default value it is
    immediately replaced.

    Parameters
    - name: The name of a node to remove.

    Return Value:
    The node removed from the map or null if no node with such a name exists.

    DOMException:
    - NOT_FOUND_ERR: Raised if there is no node named name in the map.
     */
    pub fn remove_named_item(name: DOMString) -> Node {
        unimplemented!();
    }

    /*
    Returns the indexth item in the map. If index is greater than or equal to the number of nodes
    in the map, this returns null.

    Parameters:
    - index: Index into the map.

    Return Value:
    The node at the indexth position in the NamedNodeMap, or null if that is not a valid index.

    This method raises no exceptions.
     */
    pub fn item(index: i32) -> Node {
        unimplemented!();
    }
}