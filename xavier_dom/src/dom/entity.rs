/*
Entity
--
This interface represents an entity, either parsed or unparsed, in an XML document. Note that this
models the entity itself not the entity declaration. Entity declaration modeling has been left for a
later Level of the DOM specification.

The nodeName attribute that is inherited from Node contains the name of the entity.

An XML processor may choose to completely expand entities before the structure model is passed to
the DOM; in this case there will be no EntityReference nodes in the document tree.

XML does not mandate that a non-validating XML processor read and process entity declarations made
in the external subset or declared in external parameter entities. This means that parsed entities
declared in the external subset need not be expanded by some classes of applications, and that the
replacement value of the entity may not be available. When the replacement value is available, the
corresponding Entity node's child list represents the structure of that replacement text. Otherwise,
 the child list is empty.

The resolution of the children of the Entity (the replacement value) may be lazily evaluated;
actions by the user (such as calling the childNodes method on the Entity Node) are assumed to
trigger the evaluation.

The DOM Level 1 does not support editing Entity nodes; if a user wants to make changes to the
contents of an Entity, every related EntityReference node has to be replaced in the structure
model by a clone of the Entity's contents, and then the desired changes must be made to each of
those clones instead. All the descendants of an Entity node are readonly.

An Entity node does not have any parent.

IDL Definition
interface Entity : Node {
  readonly attribute  DOMString            publicId;
  readonly attribute  DOMString            systemId;
  readonly attribute  DOMString            notationName;
};

 */

pub struct Entity {
    /*
    The public identifier associated with the entity, if specified. If the public identifier was not
    specified, this is null.
     */
    pub public_id: DOMString,
    /*
    The system identifier associated with the entity, if specified. If the system identifier was not
     specified, this is null.
     */
    pub system_id: DOMString,
    /*
    For unparsed entities, the name of the notation for the entity. For parsed entities, this is
    null.
     */
    pub notation_name: DOMString
}