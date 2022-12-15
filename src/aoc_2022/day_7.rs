use advent_of_code::Day;


enum Object<'a> {
    // Folder { name: &'a str, contents: Box<Vec<Object<'a>>> },
    // File { name: &'a str, size: u32 }
    Folder(&'a str, Box<Vec<Object<'a>>>),
    File(&'a str, u32)
}

impl<'a> Object<'a> {
    fn name(&self) -> &'a str {
        match self {
            Object::Folder(n, c) => n,
            Object::File(n, s) => n
        }
    }

    fn get_child(&mut self, name: &str) -> Option<&'a mut Object> {
        match self {
            Object::Folder(n, ref mut c) => {
                for (i, o) in (*c).iter_mut().enumerate() {
                    if o.name() == name {
                        return Some(&mut (*c)[i]);
                    }
                }
            },
            Object::File(_, _) => panic!("Files have no children")
        }

        None
    }

    fn get_by_pwd(&'a mut self, pwd: &Vec<&str>) -> &'a mut Object<'a> {
        let mut curr_obj: &mut Object<'a> = self;
        for d in pwd {
            curr_obj = curr_obj.get_child(d).unwrap();
        }
        curr_obj
    }
}

pub struct Day7 {}
impl Day for Day7 {
    fn day_number(&self) -> u8 {
        7
    }

    fn part1(&self, input: &String) -> String {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let mut pwd: Vec<&str> = Vec::new();
        let mut tree = Object::Folder("", Box::new(Vec::new()));
        // let mut pwd_obj = &mut tree;

        for line in input.lines() {
            if line.starts_with('$') {
                if &line[2..3] == "cd" {
                    match &line[5..] {
                        "/" => pwd.clear(),
                        ".." => _ = pwd.pop(),
                        other => pwd.push(other)
                    }
                }

                // pwd_obj = tree.get_by_pwd(&pwd);
            } else {
                if line.starts_with('d') { // that means it starts with dir
                    match *tree.get_by_pwd(&pwd) {
                        Object::Folder(n, c) => c.push(Object::Folder(&line[4..], Box::new(Vec::new()))),
                        Object::File(_, _) => panic!("pwd points to file")
                    }
                } else {}
            }
        }

        String::new()
    }

    fn part2(&self, input: &String) -> String {
        String::new()
    }
}
