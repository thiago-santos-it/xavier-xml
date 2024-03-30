/*
Interface Comment
--
This represents the content of a comment, i.e., all the characters between the starting
'<!--' and ending '-->'. Note that this is the definition of a comment in XML, and, in practice,
HTML, although some HTML tools may implement the full SGML comment structure.

IDL Definition
interface Comment : CharacterData { };

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */
type Comment = CharactedData;