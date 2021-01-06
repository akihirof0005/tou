use crate::structs::Atom;
use crate::structs::Point3d;

///Vec<Atom>から-NH2(アミン基)をカウントする
pub fn is_included_nh2(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "N" && atom.links.len() == 3 {
            if (tou[(atom.links[0] - 1) as usize].symbol == "C"
                && tou[(atom.links[1] - 1) as usize].symbol == "H"
                && tou[(atom.links[2] - 1) as usize].symbol == "H")
                || (tou[(atom.links[0] - 1) as usize].symbol == "H"
                    && tou[(atom.links[1] - 1) as usize].symbol == "C"
                    && tou[(atom.links[2] - 1) as usize].symbol == "H")
                || (tou[(atom.links[0] - 1) as usize].symbol == "H"
                    && tou[(atom.links[1] - 1) as usize].symbol == "H"
                    && tou[(atom.links[2] - 1) as usize].symbol == "C")
            {
                ret += 1;
            }
        }
    }
    return ret;
}

///Vec<Atom>から-OH(水酸基)をカウントする
pub fn is_included_oh(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "O" && atom.links.len() == 2 {
            if (tou[(atom.links[0] - 1) as usize].symbol == "C"
                && tou[(atom.links[1] - 1) as usize].symbol == "H")
                || (tou[(atom.links[0] - 1) as usize].symbol == "H"
                    && tou[(atom.links[1] - 1) as usize].symbol == "C")
            {
                ret += 1;
            }
        }
    }
    return ret;
}

///Vec<Atom>から-COOH(カルボン酸)をカウントする
pub fn is_included_cooh(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "O"
            && atom.links.len() == 1
            && tou[(atom.links[0] - 1) as usize].symbol == "C"
        {
            let carbon: &Atom = &tou[(atom.links[0] - 1) as usize];
            if carbon.links.len() == 3 {
                if (tou[(carbon.links[0] - 1) as usize].symbol == "C"
                    && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                    && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "C")
                {
                    ret += 1;
                }
            }
        }
    }
    return ret;
}

///Vec<Atom>から-NOCH3(Nアセチル)をカウントする
pub fn is_included_noc(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "O"
            && atom.links.len() == 1
            && tou[(atom.links[0] - 1) as usize].symbol == "C"
        {
            let carbon: &Atom = &tou[(atom.links[0] - 1) as usize];
            if carbon.links.len() == 3 {
                if (tou[(carbon.links[0] - 1) as usize].symbol == "C"
                    && tou[(carbon.links[1] - 1) as usize].symbol == "N"
                    && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "N")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "N"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "N"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "C")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "N"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "C")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "N")
                {
                    ret += 1;
                }
            }
        }
    }
    return ret;
}
