def insert(root,data):
    if(root.type=="l0"):
        if(root.data is None):
            root.data=data[0]
            if(len(data)==1):
                return
            else:
                return insert(root,data[1:])
        else:
            root = L1(root,None)

