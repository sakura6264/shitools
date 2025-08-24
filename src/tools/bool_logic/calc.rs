use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::vec::Vec;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // token of expression,LB and RB are brackets
    Var(char),
    Number(bool),
    Not,
    And,
    Or,
    LB,
    RB,
}
pub fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = expr.chars();
    let mut c = chars.next();
    while c.is_some() {
        match c {
            Some(' ') => {}
            Some('a'..='z') | Some('A'..='Z') => {
                if !tokens.is_empty() {
                    match tokens[tokens.len() - 1] {
                        Token::Var(_) | Token::RB | Token::Not | Token::Number(_) => {
                            tokens.push(Token::And)
                        }
                        _ => {}
                    }
                } // when the last token is Var, RB or Not, add And
                tokens.push(Token::Var(c.unwrap()));
            }
            Some('\'') => tokens.push(Token::Not),
            Some('+') => tokens.push(Token::Or),
            Some('(') => {
                if !tokens.is_empty() {
                    match tokens[tokens.len() - 1] {
                        Token::Var(_) | Token::RB | Token::Not | Token::Number(_) => {
                            tokens.push(Token::And)
                        }
                        _ => {}
                    }
                } // when the last token is Var, RB or Not, add And
                tokens.push(Token::LB);
            }
            Some(')') => tokens.push(Token::RB),
            Some('0') => tokens.push(Token::Number(false)),
            Some('1') => tokens.push(Token::Number(true)),
            Some(_) => return Err(format!("Invalid character: {}", c.unwrap())),
            None => {}
        }
        c = chars.next();
    }
    Ok(tokens)
}
#[derive(Clone, PartialEq, Debug)]
pub enum StackOp {
    // operator of stack
    Var(char),
    Number(bool),
    Not,
    And,
    Or,
}
pub fn getstackop(tokens: &Vec<Token>) -> Result<Vec<StackOp>, String> {
    //convert tokens to postfix expression
    let mut stack = Vec::new();
    let mut stackops = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Var(c) => stackops.push(Token::Var(c)),
            Token::Number(b) => stackops.push(Token::Number(b)),
            Token::Not => stack.push(Token::Not),
            Token::And => {
                while !stack.is_empty()
                    && stack[stack.len() - 1] != Token::Or
                    && stack[stack.len() - 1] != Token::LB
                {
                    stackops.push(stack.pop().ok_or("Stack is empty".to_string())?);
                }
                stack.push(Token::And);
            }
            Token::Or => {
                while !stack.is_empty() && stack[stack.len() - 1] != Token::LB {
                    stackops.push(stack.pop().ok_or("Stack is empty".to_string())?);
                }
                stack.push(Token::Or);
            }
            Token::LB => stack.push(Token::LB),
            Token::RB => {
                while !stack.is_empty() && stack[stack.len() - 1] != Token::LB {
                    stackops.push(stack.pop().ok_or("Stack is empty".to_string())?);
                }
                if stack.is_empty() {
                    return Err("Invalid expression".to_string());
                }
                stack.pop();
            }
        }
        i += 1;
    }
    while !stack.is_empty() {
        stackops.push(stack.pop().ok_or("Stack is empty".to_string())?);
    }
    let result = stackops
        .iter()
        .map(|x| match x {
            Token::Var(c) => StackOp::Var(*c),
            Token::Not => StackOp::Not,
            Token::And => StackOp::And,
            Token::Or => StackOp::Or,
            Token::Number(b) => StackOp::Number(*b),
            _ => unreachable!(),
        })
        .collect();
    Ok(result)
}
pub fn calculate(stackops: &Vec<StackOp>, vars: &BTreeMap<char, bool>) -> Result<bool, String> {
    // use stack to calculate the postfix expression
    let mut stack = Vec::new();
    let mut i = 0;
    while i < stackops.len() {
        match stackops[i] {
            StackOp::Var(c) => {
                stack.push(
                    *vars
                        .get(&c)
                        .ok_or(format!("Variable {} is not defined", c))?,
                );
            }
            StackOp::Number(b) => stack.push(b),
            StackOp::Not => {
                if stack.is_empty() {
                    return Err("Invalid expression".to_string());
                }
                let temp = stack.pop().ok_or("Stack is empty".to_string())?;
                stack.push(!temp);
            }
            StackOp::And => {
                if stack.len() < 2 {
                    return Err("Invalid expression".to_string());
                }
                let a = stack.pop().ok_or("Stack is empty".to_string())?;
                let b = stack.pop().ok_or("Stack is empty".to_string())?;
                stack.push(a && b);
            }
            StackOp::Or => {
                if stack.len() < 2 {
                    return Err("Invalid expression".to_string());
                }
                let a = stack.pop().ok_or("Stack is empty".to_string())?;
                let b = stack.pop().ok_or("Stack is empty".to_string())?;
                stack.push(a || b);
            }
        }
        i += 1;
    }
    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }
    Ok(stack[0])
}
pub fn getvars(stackops: &Vec<StackOp>) -> Vec<char> {
    let mut vars = Vec::new();
    for stackop in stackops {
        if let StackOp::Var(c) = stackop {
            vars.push(*c);
        }
    }
    vars.sort();
    vars
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TruthTableResult {
    Val(bool),
    NotCare,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TruthTable {
    pub vars: Vec<char>,
    pub outputs: Vec<char>,
    pub table: Vec<Vec<TruthTableResult>>,
}
pub const CHARLIST: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl TruthTable {
    pub fn next(bool_map: &mut BTreeMap<char, bool>) {
        let mut carry = true;
        for (_, value) in bool_map.iter_mut() {
            if carry {
                *value = !*value;
                carry = !*value;
            }
        }
    }
    pub fn new(inputs: usize, output: usize) -> Option<TruthTable> {
        if inputs > CHARLIST.len() || output > CHARLIST.len() {
            return None;
        }
        let mut t = Vec::new();
        for _ in 0..2usize.pow(inputs as u32) {
            let mut row = Vec::new();
            for _ in 0..output {
                row.push(TruthTableResult::Val(false));
            }
            t.push(row);
        }
        Some(TruthTable {
            vars: CHARLIST[0..inputs].to_vec(),
            outputs: CHARLIST[inputs..inputs + output].to_vec(),
            table: t,
        })
    }
    pub fn calc(
        exprs: &BTreeMap<char, String>,
        consts: &Vec<String>,
    ) -> Result<TruthTable, String> {
        if exprs.is_empty() {
            return Err("No expressions".to_string());
        }
        let expr_stackop = exprs
            .iter()
            .map(|(key, value)| {
                let tokens = tokenize(value)?;
                let stackop = getstackop(&tokens)?;
                Ok((*key, stackop))
            })
            .collect::<Result<BTreeMap<char, Vec<StackOp>>, String>>()?;
        let consts_stackop = consts
            .iter()
            .map(|value| {
                let tokens = tokenize(value)?;
                let stackop = getstackop(&tokens)?;
                Ok(stackop)
            })
            .collect::<Result<Vec<Vec<StackOp>>, String>>()?;
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for (key, value) in expr_stackop.iter() {
            for vars in getvars(value) {
                if !inputs.contains(&vars) {
                    inputs.push(vars);
                }
            }
            outputs.push(*key);
        }
        let mut truth = Vec::new();
        let mut bool_map = BTreeMap::new();
        for input in inputs.iter() {
            bool_map.insert(*input, false);
        }
        for _ in 0..2usize.pow(inputs.len() as u32) {
            let mut results = Vec::new();
            for (_, value) in expr_stackop.iter() {
                for i in consts_stackop.iter() {
                    if calculate(i, &bool_map)? {
                        results.push(TruthTableResult::NotCare);
                        continue;
                    }
                }
                let r = calculate(value, &bool_map)?;
                results.push(TruthTableResult::Val(r));
            }
            truth.push(results);
            Self::next(&mut bool_map);
        }
        Ok(Self {
            vars: inputs,
            outputs,
            table: truth,
        })
    }
}

#[inline]
pub fn get_bit(mut value: u32, pos: u32) -> bool {
    value >>= pos;
    (value & 0x1) != 0
}

#[derive(Eq, PartialEq, Hash)]
pub struct Term {
    value: u32,

    /// The mask indicates if the bit at the i position should be considered or not.
    /// The mask bits are inverted for convenience.
    ///
    /// data: 1011
    /// mask: 0001
    /// This combination refers to the min_term 101- or AB'C.
    mask: u32,
}

impl Term {
    pub fn new_with_mask(value: u32, mask: u32) -> Term {
        Term { value, mask }
    }

    pub fn new(value: u32) -> Term {
        Term::new_with_mask(value, 0)
    }

    pub fn covers(&self, term: &Term) -> bool {
        let common_mask = self.mask | term.mask;
        self.value & !common_mask == term.value & !common_mask
    }

    fn to_string(&self, names: &Vec<char>, literal_mode: bool) -> String {
        let mut result: String = String::new();
        let mut index = 0;

        let mut value = self.value;
        let mut mask = self.mask;

        for _ in 0..names.len() {
            // literal mode (write the term using letters)
            if literal_mode && mask & 1 == 0 {
                result.insert(0, names[index]);
                if value & 1 == 0 {
                    result.insert(1, '\'');
                }
            }

            // non literal mode (write the term in binary)
            if !literal_mode {
                if mask & 1 == 0 {
                    result.insert(0, if value & 1 == 0 { '0' } else { '1' });
                } else {
                    result.insert(0, '-');
                }
            }

            mask >>= 1;
            value >>= 1;

            index += 1;
        }

        result
    }

    fn terms_to_string<'a>(
        names: &Vec<char>,
        terms: impl Iterator<Item = &'a Term>,
        literal_mode: bool,
    ) -> String {
        let result: String = terms
            .map(|term| -> String { term.to_string(names, literal_mode) })
            .collect::<Vec<String>>()
            .join("+");

        result
    }
}

impl Clone for Term {
    fn clone(&self) -> Self {
        Term {
            value: self.value,
            mask: self.mask,
        }
    }
}

pub struct BinaryFunction {
    /// Number of bits
    cardinality: usize,

    terms: HashSet<Term>,
    dont_care: HashSet<Term>,
}

impl BinaryFunction {
    pub fn new(cardinality: usize) -> BinaryFunction {
        BinaryFunction {
            cardinality,
            terms: HashSet::new(),
            dont_care: HashSet::new(),
        }
    }

    pub fn add_term(&mut self, term: Term) {
        self.terms.insert(term);
    }

    pub fn get_terms(&self) -> &HashSet<Term> {
        &self.terms
    }

    pub fn add_dont_care(&mut self, term: Term) {
        self.dont_care.insert(term);
    }

    pub fn get_dont_care(&self) -> &HashSet<Term> {
        &self.dont_care
    }

    fn print_implicants(&self, implicants: &HashSet<Term>) {
        for implicant in implicants {
            println!(
                "value={:#08b} mask={:#08b}",
                implicant.value, implicant.mask
            );
        }
    }

    /// Gets the number of 1s of the given value counting only the bits that
    /// corresponds to a set mask.
    fn get_number_of_1(&self, mut value: u32, mut mask: u32) -> usize {
        let mut result = 0;
        while value != 0 {
            if (mask & 1) != 0 && (value & 1) != 0 {
                result += 1;
            }
            value >>= 1;
            mask >>= 1;
        }
        result
    }
}

pub fn qmc_find_prime_imp(
    f: &BinaryFunction,
    to_simplify: &HashSet<Term>,
) -> (HashSet<Term>, usize) {
    //print!("[QMC] To simplify: {}\n", Term::terms_to_string(f.cardinality, to_simplify.iter(), false));

    let groups_num = f.cardinality + 1;
    let mut groups: Vec<Vec<(&Term, bool)>> = vec![Vec::new(); groups_num];
    for imp in to_simplify.iter() {
        let num_of_1 = f.get_number_of_1(imp.value, !imp.mask);
        groups[num_of_1].push((imp, false));
    }
    /*
        for num_of_1 in 0..groups_num {
            print!("[QMC] Group {}: {}\n", num_of_1, Term::terms_to_string(
                f.cardinality,
                groups[num_of_1].iter().map(|(term, _)| (*term)),
                false
            ));
        }
    */
    let mut result: HashSet<Term> = HashSet::new();
    let mut simplified_num = 0;

    for i in 0..groups.len() {
        let j = i + 1;

        let (i_group, j_group) = groups.split_at_mut(j);

        for (imp_a, a_simplified) in i_group.last_mut().unwrap() {
            if j < groups_num {
                for (imp_b, b_simplified) in j_group.first_mut().unwrap() {
                    // The two implicants must share the same mask:
                    // 10-- | 00-- | -00-
                    // -001 | 01-- | -11-
                    // NO   | YES  | YES  ...
                    if imp_a.mask != imp_b.mask {
                        continue;
                    }

                    // Gets the differing 1s from both implicants, if the number of differing 1s is 1,
                    // then we can create a new simplified term.
                    // 10  | 00
                    // 01  | 01
                    // 11  | 01
                    // NO  | YES ...
                    let mask = imp_a.mask; // = imp_b.mask
                    let diff = imp_a.value ^ imp_b.value;
                    if f.get_number_of_1(diff, !mask) == 1 {
                        let simplified_imp = Term::new_with_mask(imp_a.value, mask | diff);

                        result.insert(simplified_imp);

                        *a_simplified = true;
                        *b_simplified = true;
                    }
                }
            }

            // If the implicant hasn't been simplified put it in the resulting vector.
            // 000 | 00- | 00- | 00-
            // 001 |     | 001 | 001
            // 111 |     |     | 111
            // 001 couldn't get simplified because varies two 1s compared to 111.
            if !*a_simplified {
                result.insert((*imp_a).clone());
            } else {
                simplified_num += 1;
            }
        }
    }

    //print!("[QMC] Simplified: {}\n", simplified_num);

    if simplified_num > 0 {
        result = qmc_find_prime_imp(f, &result).0;
    }

    (result, simplified_num)
}

pub fn qmc_find_prime_imp_from_func(f: &BinaryFunction) -> HashSet<Term> {
    // Initially implicants are taken both from min_terms and dont_care, and they're cloned inside one single vector.
    let mut imp: HashSet<Term> = HashSet::new();
    imp.extend(f.terms.iter().cloned());
    imp.extend(f.dont_care.iter().cloned());

    qmc_find_prime_imp(f, &imp).0
}

pub fn qmc_dominance_crit(
    rows: &mut HashSet<Term>,
    cols: &HashSet<Term>,
    covers: impl Fn(&Term, &Term) -> bool,
) -> usize {
    let mut to_del_rows: HashSet<Term> = HashSet::new();

    for row_1 in rows.iter() {
        for row_2 in rows.iter() {
            let was_del = to_del_rows.contains(row_1) || to_del_rows.contains(row_2); // One of the rows was simplified before!
            if !was_del && row_1 != row_2 {
                // row_1 is the one considered dominant.
                // row_2 is the one to check against.

                let mut is_dominant = true;
                for col in cols.iter() {
                    if !covers(row_1, col) && covers(row_2, col) {
                        is_dominant = false; // The row can't be dominant in this case.
                        break;
                    }
                }

                if is_dominant {
                    to_del_rows.insert(row_2.clone());
                }
            }
        }
    }

    for to_del_row in to_del_rows.iter() {
        rows.remove(to_del_row);
    }

    to_del_rows.len()
}

pub fn qmc_essentiality_crit(rows: &mut HashSet<Term>, cols: &mut HashSet<Term>) -> HashSet<Term> {
    let mut to_del_rows: HashSet<Term> = HashSet::new();
    let mut to_del_cols: HashSet<Term> = HashSet::new();

    let mut result = HashSet::new();

    for col in cols.iter() {
        if to_del_cols.contains(col) {
            continue;
        }

        for row_1 in rows.iter() {
            if to_del_rows.contains(row_1) {
                continue;
            }

            // Check if the current column is covered only by the row_1.
            let mut is_covered_only_by_me = row_1.covers(col);

            if is_covered_only_by_me {
                for row_2 in rows.iter() {
                    if to_del_rows.contains(row_2) {
                        continue;
                    }

                    if row_1 != row_2 && row_2.covers(col) {
                        is_covered_only_by_me = false;
                        break;
                    }
                }
            }

            // Remove the row and all the cols it covers from the given `rows` and `cols`.
            if is_covered_only_by_me {
                result.insert(row_1.clone());

                to_del_rows.insert(row_1.clone());
                for col in cols.iter() {
                    if row_1.covers(col) {
                        to_del_cols.insert(col.clone()); // Try to insert it even if it could be already there.
                    }
                }
            }
        }
    }

    for to_del_row in to_del_rows {
        rows.remove(&to_del_row);
    }

    for to_del_col in to_del_cols {
        cols.remove(&to_del_col);
    }

    result
}

pub fn qmc_find_essential_imp(f: &BinaryFunction, prime_imp: &HashSet<Term>) -> HashSet<Term> {
    let mut rows = prime_imp.clone();
    let mut cols = f.terms.clone();

    let mut result = HashSet::new();

    loop {
        // Using just the essentiality crit is enough to get the essential implicants.
        // The row/col dominant crit could be used but _just one_ of them, not both in the same process.
        /*
        loop {
            let mut simplified = false;
            simplified |= qmc_dominance_crit(&mut rows, &cols, |row, col| row.covers(col)) > 0; // Just use row dominance crit.
            //simplified |= qmc_dominance_crit(&mut cols, &rows, |col, row| row.covers(col)) > 0;
            if !simplified {
                break;
            }
        }
         */

        let found = qmc_essentiality_crit(&mut rows, &mut cols);
        if found.is_empty() {
            break;
        }
        result.extend(found);
    }

    result
}
pub fn qmc_simplify(truthtable: &TruthTable) -> BTreeMap<char, String> {
    let mut result = BTreeMap::new();
    let mut output_index = 0;
    while output_index < truthtable.outputs.len() {
        let mut bool_map = BTreeMap::new();
        let mut f = BinaryFunction::new(truthtable.vars.len());
        for i in truthtable.vars.iter() {
            bool_map.insert(*i, false);
        }
        for j in 0..2usize.pow(truthtable.vars.len() as u32) {
            let r = truthtable.table[j][output_index].clone();

            match r {
                TruthTableResult::Val(true) => {
                    let mut bin = 0;
                    for (_, v) in bool_map.iter().rev() {
                        bin <<= 1;
                        if *v {
                            bin += 1;
                        }
                    }
                    f.add_term(Term::new(bin));
                }
                TruthTableResult::NotCare => {
                    let mut bin = 0;
                    for (_, v) in bool_map.iter().rev() {
                        bin <<= 1;
                        if *v {
                            bin += 1;
                        }
                    }
                    f.add_dont_care(Term::new(bin));
                }
                _ => {}
            }
            TruthTable::next(&mut bool_map);
        }
        let prime_imp = qmc_find_prime_imp_from_func(&f);
        let essential_imp = qmc_find_essential_imp(&f, &prime_imp);
        let mut s = Term::terms_to_string(&truthtable.vars, essential_imp.iter(), true);
        if s.is_empty() {
            for i in truthtable.table.iter() {
                if i[output_index] == TruthTableResult::Val(true) {
                    s = "1".to_string();
                    break;
                } else if i[output_index] == TruthTableResult::Val(false) {
                    s = "0".to_string();
                    break;
                }
            }
        }
        result.insert(truthtable.outputs[output_index], s);
        output_index += 1;
    }
    result
}
#[derive(Clone, PartialEq, Debug)]
pub struct Graph {
    pub nodes: BTreeMap<String, String>,
    pub edges: Vec<(String, String, String)>,
}
impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: BTreeMap::new(),
            edges: Vec::new(),
        }
    }
    pub fn to_dot(&self) -> String {
        let mut result = "digraph {\n".to_string();
        for (i, node) in self.nodes.iter() {
            result += &format!("  {} [label=\"{}\"];\n", i, node);
        }
        for (from, to, label) in self.edges.iter() {
            result += &format!("  {} -> {} [label=\"{}\"];\n", from, to, label);
        }
        result += "}\n";
        result
    }
}
pub fn boollist2str(list: &Vec<bool>) -> String {
    let mut result = String::new();
    for i in list.iter() {
        if *i {
            result += "1";
        } else {
            result += "0";
        }
    }
    result
}
pub fn boollist2str_witherror(list: &Vec<TruthTableResult>) -> Result<String, String> {
    let mut result = String::new();
    for i in list.iter() {
        match i {
            TruthTableResult::Val(true) => result += "1",
            TruthTableResult::Val(false) => result += "0",
            _ => return Err("Invalid TruthTableResult".to_string()),
        }
    }
    Ok(result)
}
pub fn resultlist2str(list: &Vec<TruthTableResult>) -> String {
    let mut result = String::new();
    for i in list.iter() {
        match i {
            TruthTableResult::Val(true) => result += "1",
            TruthTableResult::Val(false) => result += "0",
            TruthTableResult::NotCare => result += "x",
        }
    }
    result
}

#[cfg(test)]
mod test {

    #[test]
    fn test_tokenize() {
        assert_eq!(
            super::tokenize("a+b"),
            Ok(vec![
                super::Token::Var('a'),
                super::Token::Or,
                super::Token::Var('b')
            ])
        );
        assert_eq!(
            super::tokenize("a'"),
            Ok(vec![super::Token::Var('a'), super::Token::Not])
        );
        assert_eq!(
            super::tokenize("a+b'"),
            Ok(vec![
                super::Token::Var('a'),
                super::Token::Or,
                super::Token::Var('b'),
                super::Token::Not
            ])
        );
        assert_eq!(
            super::tokenize("a'b"),
            Ok(vec![
                super::Token::Var('a'),
                super::Token::Not,
                super::Token::And,
                super::Token::Var('b')
            ])
        );
        assert_eq!(
            super::tokenize("(ab)'c"),
            Ok(vec![
                super::Token::LB,
                super::Token::Var('a'),
                super::Token::And,
                super::Token::Var('b'),
                super::Token::RB,
                super::Token::Not,
                super::Token::And,
                super::Token::Var('c')
            ])
        );
    }
    #[test]
    fn test_stackop() {
        let tk = super::tokenize("d+(a+b)c").unwrap();
        let ops = super::getstackop(&tk).unwrap();
        assert_eq!(
            ops,
            vec![
                super::StackOp::Var('d'),
                super::StackOp::Var('a'),
                super::StackOp::Var('b'),
                super::StackOp::Or,
                super::StackOp::Var('c'),
                super::StackOp::And,
                super::StackOp::Or,
            ]
        );
    }
    #[test]
    fn test_calc() {
        let tk = super::tokenize("a+b'c").unwrap();
        let ops = super::getstackop(&tk).unwrap();
        let result = super::calculate(
            &ops,
            &[('a', true), ('b', false), ('c', true)]
                .iter()
                .cloned()
                .collect(),
        )
        .unwrap();
        assert_eq!(result, true);
    }
    #[test]
    fn test_qm() {
        let mut f = super::BinaryFunction::new(4); // The `cardinality` tells how many inputs do we have.

        // Here define the inputs combination that will give f(input) = 1.
        // Giving in f.add_term(5), will result in f(0101) = 1. Not defined inputs will result in f(not_def_input) = 0.
        f.add_term(super::Term::new(0));
        f.add_term(super::Term::new(1));
        f.add_term(super::Term::new(2));
        f.add_term(super::Term::new(3));

        // Here define the inputs combination that will give f(input) = x (or dont_care).
        // Giving in f.add_term(10), will result in f(1010) = x.
        /*
            f.add_dont_care(super::Term::new(10));
            f.add_dont_care(super::Term::new(11));
        */
        // This is the function that will execute the algorithm and takes care of printing the results.
        let prime_imp = super::qmc_find_prime_imp_from_func(&f);
        assert_eq!(
            "d'c'",
            super::Term::terms_to_string(&vec!['a', 'b', 'c', 'd'], prime_imp.iter(), true)
        );

        let essential_imp = super::qmc_find_essential_imp(&f, &prime_imp);
        assert_eq!(
            "D'c'",
            super::Term::terms_to_string(&vec!['A', 'b', 'c', 'D'], essential_imp.iter(), true)
        );
    }
}
