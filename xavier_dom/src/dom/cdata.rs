/*
Extended (Not implemented yet!)
--
CDATA sections are used to escape blocks of text containing characters that would otherwise be
regarded as markup. The only delimiter that is recognized in a CDATA section is the "]]>" string
that ends the CDATA section. CDATA sections can not be nested. The primary purpose is for including
material such as XML fragments, without needing to escape all the delimiters.

The DOMString attribute of the Text node holds the text that is contained by the CDATA section.
Note that this may contain characters that need to be escaped outside of CDATA sections and that,
depending on the character encoding ("charset") chosen for serialization, it may be impossible to
write out some characters as part of a CDATA section.

The CDATASection interface inherits the CharacterData interface through the Text interface. Adjacent
CDATASections nodes are not merged by use of the Element.normalize() method.

IDL Definition
interface CDATASection : Text {
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */
type CDATASection = Text;