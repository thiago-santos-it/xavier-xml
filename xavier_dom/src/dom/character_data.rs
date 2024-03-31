/*
Interface CharacterData
--
The CharacterData interface extends Node with a set of attributes and methods for accessing
character data in the DOM. For clarity this set is defined here rather than on each object that uses
these attributes and methods. No DOM objects correspond directly to CharacterData, though Text and
others do inherit the interface from it. All offsets in this interface start from 0.

IDL Definition
interface CharacterData : Node {
           attribute  DOMString            data;
                                 // raises(DOMException) on setting
                                 // raises(DOMException) on retrieval
  readonly attribute  unsigned long        length;
  DOMString                 substringData(in unsigned long offset,
                                          in unsigned long count)
                                          raises(DOMException);
  void                      appendData(in DOMString arg)
                                       raises(DOMException);
  void                      insertData(in unsigned long offset,
                                       in DOMString arg)
                                       raises(DOMException);
  void                      deleteData(in unsigned long offset,
                                       in unsigned long count)
                                       raises(DOMException);
  void                      replaceData(in unsigned long offset,
                                        in unsigned long count,
                                        in DOMString arg)
                                        raises(DOMException);
};

DOMException:

NO_MODIFICATION_ALLOWED_ERR: Raised when the node is readonly.

DOMSTRING_SIZE_ERR: Raised when it would return more characters than fit in a DOMString variable on
the implementation platform.

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */

pub struct CharacterData {

    /*
    The character data of the node that implements this interface. The DOM implementation may not
    put arbitrary limits on the amount of data that may be stored in a CharacterData node. However,
    implementation limits may mean that the entirety of a node's data may not fit into a single
    DOMString. In such cases, the user may call substringData to retrieve the data in appropriately
    sized pieces.
     */
    pub data: DOMString,
    /*
    The number of characters that are available through data and the substringData method below. This
    may have the value zero, i.e., CharacterData nodes may be empty.
     */
    pub length: i32
}


trait CharacterDataTrait {
    /*
    Extracts a range of data from the node.

    Parameters:
    - offset: Start offset of substring to extract.
    - count: The number of characters to extract.

    Return Value
    The specified substring. If the sum of offset and count exceeds the length, then all characters
    to the end of the data are returned.

    DOMException:
    - INDEX_SIZE_ERR: Raised if the specified offset is negative or greater than the number of
    characters in data, or if the specified count is negative.
    - DOMSTRING_SIZE_ERR: Raised if the specified range of text does not fit into a DOMString.
     */
    fn substring_data(offset: i32, count: i32) -> DOMString;

    /*
    Append the string to the end of the character data of the node. Upon success, data provides
    access to the concatenation of data and the DOMString specified.

    Parameters:
    - arg: The DOMString to append.

    DOMException:
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

    This method returns nothing.
     */
    fn append_data(arg: DOMString);

    /*
    Insert a string at the specified character offset.

    Parameters:
    - offset: The character offset at which to insert.
    - arg: The DOMString to insert.

    DOMException:
    - INDEX_SIZE_ERR: Raised if the specified offset is negative or greater than the number of
    characters in data.
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

    This method returns nothing.
     */
    fn insert_data(offset: i32, arg: DOMString);

    /*
    Remove a range of characters from the node. Upon success, data and length reflect the change.
    Parameters:
    - offset: The offset from which to remove characters.
    - count: The number of characters to delete. If the sum of offset and count exceeds length then
    all characters from offset to the end of the data are deleted.

    DOMException:
    - INDEX_SIZE_ERR: Raised if the specified offset is negative or greater than the number of
    characters in data, or if the specified count is negative.
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

     */
    fn delete_data(offset: i32, count: i32);

    /*
    Replace the characters starting at the specified character offset with the specified string.
    Parameters:
    - offset: The offset from which to start replacing.
    - count: The number of characters to replace. If the sum of offset and count exceeds length,
    then all characters to the end of the data are replaced (i.e., the effect is the same as a
    remove method call with the same range, followed by an append method invocation).
    - arg: The DOMString with which the range must be replaced.

    DOMException:
    - INDEX_SIZE_ERR: Raised if the specified offset is negative or greater than the number of
    characters in data, or if the specified count is negative.
    - NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.

    This method returns nothing.
     */
    fn replace_data(offset: i32, count: i32, arg: DOMString);
}

impl CharacterDataTrait for CharacterData {

    fn delete_data(offset: i32, count: i32) {
        unimplemented!()
    }

    fn length(&self) -> i32 {
        unimplemented!()
    }

    fn substring_data(offset: i32, count: i32) -> DOMString {
        unimplemented!()
    }

    fn append_data(arg: DOMString) {
        unimplemented!()
    }

    fn insert_data(offset: i32, arg: DOMString) {
        unimplemented!()
    }

    fn replace_data(offset: i32, count: i32, arg: DOMString) {
        unimplemented!()
    }
}