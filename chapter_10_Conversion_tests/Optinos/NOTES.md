

# BOX => 
  - when we use someitng in box Box<Node>
    - we can directly do Node.value or Node.next ? why 

###  Box property De-ref Co-Er-cion
mean &mut Box<node>  becomes node.value

inside rust does.  (*Node).value


# take()
  - if we do  
    - let x = node.next;
    - node.next => will move to x but "Partially".
    - as stracturall its there, but the value is not there.
  - take solves this with immideately replacing value with None.
    - the direct move will anyways fail as rust prevents 
    - "moving owned fields,out of borrowed containt"
    - this meke sures all values remains fully valid.



