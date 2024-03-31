/*
Interface Element
--

By far the vast majority of objects (apart from text) that authors encounter when traversing a
document are Element nodes. Assume the following XML document:
``` xml
<elementExample id="demo">
  <subelement1/>
  <subelement2><subsubelement/></subelement2>
</elementExample>
```

When represented using DOM, the top node is an Element node for "elementExample", which contains
two child Element nodes, one for "subelement1" and one for "subelement2". "subelement1" contains no
child nodes.

Elements may have attributes associated with them; since the Element interface inherits from Node,
the generic Node interface method getAttributes may be used to retrieve the set of all attributes
for an element. There are methods on the Element interface to retrieve either an Attr object by
name or an attribute value by name. In XML, where an attribute value may contain entity references,
an Attr object should be retrieved to examine the possibly fairly complex sub-tree representing
the attribute value. On the other hand, in HTML, where all attributes have simple string values,
 methods to directly access an attribute value can safely be used as a convenience.

IDL Definition
interface Element : Node {
  readonly attribute  DOMString            tagName;
  DOMString                 getAttribute(in DOMString name);
  void                      setAttribute(in DOMString name,
                                         in DOMString value)
                                         raises(DOMException);
  void                      removeAttribute(in DOMString name)
                                            raises(DOMException);
  Attr                      getAttributeNode(in DOMString name);
  Attr                      setAttributeNode(in Attr newAttr)
                                             raises(DOMException);
  Attr                      removeAttributeNode(in Attr oldAttr)
                                                raises(DOMException);
  NodeList                  getElementsByTagName(in DOMString name);
  void                      normalize();
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */

use crate::dom::attr::Attr;
use crate::dom::node::{NodeImpl, NodeTrait};
use crate::dom::node_list::NodeList;
use crate::dom::string::DOMString;

pub struct Element<'a> {
    /*
    tagName: The name of the element. For example, in:

    ``` xml
    <elementExample id="demo">
        ...
    </elementExample>,
    ```
    tagName has the value "elementExample". Note that this is case-preserving in XML, as are all of
    the operations of the DOM. The HTML DOM returns the tagName of an HTML element in the canonical
    uppercase form, regardless of the case in the source HTML document.
    */
    pub tag_name: DOMString,
    /*
    Inner state of node
     */
    inner: NodeImpl<'a>
}

impl Element<'_> {

    /*
    Retrieves an attribute value by name.

    Parameters:
    - name: The name of the attribute to retrieve.

    Return Value:
    The Attr value as a string, or the empty string if that attribute does not have a specified or
    default value.

    This method raises no exceptions.
     */
    pub fn get_attribute(name: DOMString) {
        unimplemented!()
    }

    /*
    Adds a new attribute. If an attribute with that name is already present in the element, its
    value is changed to be that of the value parameter. This value is a simple string, it is not
    parsed as it is being set. So any markup (such as syntax to be recognized as an entity
    reference) is treated as literal text, and needs to be appropriately escaped by the
    implementation when it is written out. In order to assign an attribute value that contains
    entity references, the user must create an Attr node plus any Text and EntityReference nodes,
    build the appropriate subtree, and use setAttributeNode to assign it as the value of an
    attribute.

    Parameters:
    - name: The name of the attribute to create or alter.
    - value: Value to set in string form.

    DOMException:
    - INVALID_CHARACTER_ERR: Raised if the specified name contains an invalid character.
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

    This method returns nothing.
     */
    pub fn set_attribute(name: DOMString, value: DOMString) {
        unimplemented!()
    }

    /*
    Removes an attribute by name. If the removed attribute has a default value it is immediately
    replaced.

    Parameters:
    - name: The name of the attribute to remove.

    DOMException:
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

    This method returns nothing.
     */
    pub fn remove_attribute(name: DOMString) {
        unimplemented!()
    }
    /*
    Retrieves an Attr node by name.
    Parameters:
    - name: The name of the attribute to retrieve.

    Return Value:
    The Attr node with the specified attribute name or null if there is no such attribute.

    This method raises no exceptions.
     */
    pub fn get_attribute_node(name: DOMString) -> Attr {
        unimplemented!()
    }

    /*
    Adds a new attribute. If an attribute with that name is already present in the element, it is
    replaced by the new one.
    Parameters:
    - newAttr: The Attr node to add to the attribute list.

    Return Value:
    If the newAttr attribute replaces an existing attribute with the same name, the previously
    existing Attr node is returned, otherwise null is returned.

    DOMException:
    - WRONG_DOCUMENT_ERR: Raised if newAttr was created from a different document than the one that
    created the element.

    NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

    INUSE_ATTRIBUTE_ERR: Raised if newAttr is already an attribute of another Element object. The
    DOM user must explicitly clone Attr nodes to re-use them in other elements.
     */
    pub fn set_attribute_node(new_attr: Attr) -> Attr {
        unimplemented!()
    }

    /*
    Removes the specified attribute.
    Parameters:
    - oldAttr: The Attr node to remove from the attribute list. If the removed Attr has a default
     value it is immediately replaced.

    Return Value:
    The Attr node that was removed.

    DOMException:
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.
    - NOT_FOUND_ERR: Raised if oldAttr is not an attribute of the element.
     */
    pub fn remove_attribute_node(old_attr: Attr) -> Attr {
        unimplemented!()
    }

    /*
    Returns a NodeList of all descendant elements with a given tag name, in the order in which they would be encountered in a preorder traversal of the Element tree.
    Parameters
    - name: The name of the tag to match on. The special value "*" matches all tags.

    Return Value:
    A list of matching Element nodes.

    This method raises no exceptions.
     */
    pub fn get_elements_by_tag_name(name: DOMString) -> NodeList {
        unimplemented!()
    }

    /*
    Puts all Text nodes in the full depth of the sub-tree underneath this Element into a "normal"
    form where only markup (e.g., tags, comments, processing instructions, CDATA sections, and
    entity references) separates Text nodes, i.e., there are no adjacent Text nodes. This can be
    used to ensure that the DOM view of a document is the same as if it were saved and re-loaded,
    and is useful when operations (such as XPointer lookups) that depend on a particular document
    tree structure are to be used.

    This method has no parameters.
    This method returns nothing.
    This method raises no exceptions.
     */
    pub fn normalize() {
        unimplemented!()
    }
}

impl NodeTrait for Element<'_> {
    fn inner(&mut self) -> &mut NodeImpl {
        //&mut self.inner
        unimplemented!()
    }
}
