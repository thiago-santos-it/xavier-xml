/*
Interface Text
--
The Text interface represents the textual content (termed character data in XML) of an Element or Attr. If there is no markup inside an element's content, the text is contained in a single object implementing the Text interface that is the only child of the element. If there is markup, it is parsed into a list of elements and Text nodes that form the list of children of the element.

When a document is first made available via the DOM, there is only one Text node for each block of text. Users may create adjacent Text nodes that represent the contents of a given element without any intervening markup, but should be aware that there is no way to represent the separations between these nodes in XML or HTML, so they will not (in general) persist between DOM editing sessions. The normalize() method on Element merges any such adjacent Text objects into a single node for each block of text; this is recommended before employing operations that depend on a particular document structure, such as navigation with XPointers.

IDL Definition
interface Text : CharacterData {
  Text                      splitText(in unsigned long offset)
                                      raises(DOMException);
};

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */
use crate::dom::character_data::{CharacterData, CharacterDataTrait};


pub struct Text {
    inner: CharacterData
}

impl CharacterDataTrait for Text {
    fn inner(&mut self) -> &mut CharacterData {
        &mut self.inner
    }
}

impl Text {
    /*
    Breaks this Text node into two Text nodes at the specified offset, keeping both in the tree as
    siblings. This node then only contains all the content up to the offset point. And a new Text
    node, which is inserted as the next sibling of this node, contains all the content at and after
    the offset point.

    Parameters:
    - offset: The offset at which to split, starting from 0.

    Return Value:
    The new Text node.

    DOMException:
    - INDEX_SIZE_ERR: Raised if the specified offset is negative or greater than the number of
    characters in data.
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.
     */
    fn split_text(&self, offset: i32) -> Text {
        unimplemented!()
    }
}