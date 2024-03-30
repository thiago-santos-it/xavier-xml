/*
Definition group ExceptionCode
An integer indicating the type of error generated.

Defined Constants
INDEX_SIZE_ERR
If index or size is negative, or greater than the allowed value

DOMSTRING_SIZE_ERR
If the specified range of text does not fit into a DOMString

HIERARCHY_REQUEST_ERR
If any node is inserted somewhere it doesn't belong

WRONG_DOCUMENT_ERR
If a node is used in a different document than the one that created it (that doesn't support it)

INVALID_CHARACTER_ERR
If an invalid character is specified, such as in a name.

NO_DATA_ALLOWED_ERR
If data is specified for a node which does not support data

NO_MODIFICATION_ALLOWED_ERR
If an attempt is made to modify an object where modifications are not allowed

NOT_FOUND_ERR
If an attempt was made to reference a node in a context where it does not exist

NOT_SUPPORTED_ERR
If the implementation does not support the type of object requested

INUSE_ATTRIBUTE_ERR
If an attempt is made to add an attribute that is already inuse elsewhere
 */
