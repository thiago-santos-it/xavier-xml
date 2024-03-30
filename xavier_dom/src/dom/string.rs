/*
WARNING: The original spec says that this type holds a UTF-16 STRING, since Rust works better with
UTF-8 and the underling libs are working with UTF-8 these implementations uses the Rust standard.

From: https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html
 */
type DOMString = String;