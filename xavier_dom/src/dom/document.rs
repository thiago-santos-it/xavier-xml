/*
Interface Document
--
The Document interface represents the entire HTML or XML document. Conceptually, it is the root of the document tree, and provides the primary access to the document's data.

Since elements, text nodes, comments, processing instructions, etc. cannot exist outside the context of a Document, the Document interface also contains the factory methods needed to create these objects. The Node objects created have a ownerDocument attribute which associates them with the Document within whose context they were created.

IDL Definition
interface Document : Node {
  readonly attribute  DocumentType         doctype;
  readonly attribute  DOMImplementation    implementation;
  readonly attribute  Element              documentElement;
  Element                   createElement(in DOMString tagName)
                                          raises(DOMException);
  DocumentFragment          createDocumentFragment();
  Text                      createTextNode(in DOMString data);
  Comment                   createComment(in DOMString data);
  CDATASection              createCDATASection(in DOMString data)
                                               raises(DOMException);
  ProcessingInstruction     createProcessingInstruction(in DOMString target,
                                                        in DOMString data)
                                                        raises(DOMException);
  Attr                      createAttribute(in DOMString name)
                                            raises(DOMException);
  EntityReference           createEntityReference(in DOMString name)
                                                  raises(DOMException);
  NodeList                  getElementsByTagName(in DOMString tagname);
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */

use crate::dom::attr::Attr;
use crate::dom::cdata::CDATASection;
use crate::dom::comment::Comment;
use crate::dom::doc_type::DocumentType;
use crate::dom::element::Element;
use crate::dom::entity_ref::EntityReference;
use crate::dom::fragment::DocumentFragment;
use crate::dom::implementation::DOMImplementation;
use crate::dom::instruction::ProcessingInstruction;
use crate::dom::node::{NodeImpl, NodeTrait};
use crate::dom::node_list::NodeList;
use crate::dom::string::DOMString;
use crate::dom::text::Text;

pub struct Document<'a> {
    /*
    The Document Type Declaration (see DocumentType) associated with this document. For HTML
    documents as well as XML documents without a document type declaration this returns null.

    The DOM Level 1 does not support editing the Document Type Declaration, therefore docType
    cannot be altered in any way.
     */
    pub doctype: DocumentType<'a>,
    /*
    The DOMImplementation object that handles this document. A DOM application may use objects
    from multiple implementations.
     */
    pub implementation: DOMImplementation,
    /*
    This is a convenience attribute that allows direct access to the child node that is the root
    element of the document. For HTML documents, this is the element with the tagName "HTML".
     */
    pub document_element: Element<'a>,
    /*
    Inner state of node
     */
    inner: NodeImpl<'a>
}

impl Document<'_> {

    /*
    Creates an element of the type specified. Note that the instance returned implements the Element
    interface, so attributes can be specified directly on the returned object.

    Parameters:
    tagName: The name of the element type to instantiate. For XML, this is case-sensitive. For HTML,
    the tagName parameter may be provided in any case, but it must be mapped to the canonical
    uppercase form by the DOM implementation.

    Return Value:
    A new Element object.

    DOMException:
    - INVALID_CHARACTER_ERR: Raised if the specified name contains an invalid character.
     */
    fn create_element(tag_name: DOMString) -> Element<'static> {
        unimplemented!()
    }

    /*
    Creates an empty DocumentFragment object.

    Return Value:
    A new DocumentFragment.

    This method has no parameters.
    This method raises no exceptions.
     */
    fn create_document_fragment() -> DocumentFragment<'static> {
        unimplemented!()
    }

    /*
    Creates a Text node given the specified string.

    Parameters:
    - data: The data for the node.

    Return Value:
    The new Text object.

    This method raises no exceptions.
     */
    fn create_text_node(data: DOMString) -> Text {
        unimplemented!()
    }
    /*
    Creates a Comment node given the specified string.

    Parameters:
    - data: The data for the node.

    Return Value:
    The new Comment object.

    This method raises no exceptions.
     */
    fn  create_comment(data: DOMString) -> Comment {
        unimplemented!()
    }

    /*
    Creates a CDATASection node whose value is the specified string.

    Parameters:
    - data: The data for the CDATASection contents.

    Return Value:
    The new CDATASection object.

    DOMException:
    - NOT_SUPPORTED_ERR: Raised if this document is an HTML document.
     */
    fn create_cdata_section(data: DOMString) -> CDATASection {
        unimplemented!()
    }
/*
    Creates a ProcessingInstruction node given the specified name and data strings.

    Parameters:
    - target:The target part of the processing instruction.
    - data: The data for the node.

    Return Value:
    The new ProcessingInstruction object.

    DOMException:
    - INVALID_CHARACTER_ERR: Raised if an invalid character is specified.
    - NOT_SUPPORTED_ERR: Raised if this document is an HTML document.
    */
    fn create_processing_instruction(target: DOMString, data: DOMString) -> ProcessingInstruction<'static> {
        unimplemented!()
    }
    /*
    Creates an Attr of the given name. Note that the Attr instance can then be set on an Element
    using the setAttribute method.

    Parameters:
    - name: The name of the attribute.

    Return Value:
    A new Attr object.

    DOMException:
    - INVALID_CHARACTER_ERR: Raised if the specified name contains an invalid character.
     */
    fn create_attribute(name: DOMString) -> Attr {
        unimplemented!()
    }

    /*
    Creates an EntityReference object.

    Parameters:
    - name: The name of the entity to reference.

    Return Value:
    The new EntityReference object.

    DOMException:
    - INVALID_CHARACTER_ERR: Raised if the specified name contains an invalid character.
    - NOT_SUPPORTED_ERR: Raised if this document is an HTML document.
     */
    fn create_entity_reference(name: DOMString) -> EntityReference<'static> {
        unimplemented!()
    }
    /*
    Returns a NodeList of all the Elements with a given tag name in the order in which they would be
    encountered in a preorder traversal of the Document tree.

    Parameters:
    - tagname: The name of the tag to match on. The special value "*" matches all tags.

    Return Value:
    A new NodeList object containing all the matched Elements.

    This method raises no exceptions.
     */
    fn get_elements_by_tag_name(tagname: DOMString) -> NodeList {
        unimplemented!()
    }
}

impl NodeTrait for Document<'_> {
    fn inner(&mut self) -> &mut NodeImpl {
        //&mut self.inner
        unimplemented!()
    }
}
