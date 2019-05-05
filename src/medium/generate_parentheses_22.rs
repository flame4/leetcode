use crate::Solution;

// 标记栈状态.
#[derive(Debug, Clone)]
struct Status22 {
    l_p: i32,
    r_p: i32,
    // 0表示下一个'('还没入
    // 1表示下一个'('已经入过了, ')'还没入
    // 2表示都入过了, 该退出了.
    status: i8,
}

impl Solution {
    /// https://leetcode.com/problems/generate-parentheses/
    /// 给定一个整数n，返回一组n个括号可以组成的任何合法字符串, 表现为必须为首尾包含的嵌套关系.
    /// 思路看题中图容易想到这是用栈来模拟递归的做法.
    /// 然后每次针对产生一个新的左括号还是一个右括号来做空间搜索.
    /// 因为可能会爆栈，所以要用stack模拟递归的写法.
    /// 使用栈的写法类似下面：
    /// G(stack, l_p, r_p, result) {
    ///     if r_p > l_p : return
    ///
    ///     if l_p != 0 {
    ///         stack.push('(');
    ///         G(stack, l_p-1, r_p,result);
    ///         stack.pop();
    ///     }
    ///
    ///     if r_p != 0 {
    ///         stack.push(')');
    ///         G(stack, l_p, r_p-1, result);
    ///         stack.pop();
    ///     }
    ///
    ///     if both 0 {
    ///         result.append(stack.clone());
    ///     }
    /// }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = Vec::new();
        let mut stack = Vec::new();
        // 这个用来存储循环模拟递归的状态.
        let mut status_stack = Vec::new();
        let l_p = n;
        let r_p = n;

        status_stack.push(Status22 {
            l_p,
            r_p,
            status: 0,
        });
        while !status_stack.is_empty() {
//            println!("This loop Begin: d = {:?}, stack = {:?}", status_stack.clone(), unsafe {
//                String::from_utf8_unchecked(stack.clone())
//            });
            let d = status_stack.last_mut().unwrap();
            if d.l_p < 0 || d.r_p < 0 || d.r_p < d.l_p {
                // 无效状态, 退出.
                status_stack.pop();
                stack.pop();
                continue;
            }

            if d.l_p == 0 && d.r_p == 0 {
                // 到达一种合法输出，记得输出.
                unsafe {
                    ret.push(String::from_utf8_unchecked(stack.clone()));
                }
                status_stack.pop();
                stack.pop();
                continue;
            }

            match d.status {
                0 => {
                    stack.push('(' as u8);
                    d.status = 1;
                    let new_status = Status22 {
                        l_p: d.l_p - 1,
                        r_p: d.r_p,
                        status: 0,
                    };
                    status_stack.push(new_status);
                }
                1 => {
                    stack.push(')' as u8);
                    d.status = 2;
                    let new_status = Status22 {
                        l_p: d.l_p,
                        r_p: d.r_p - 1,
                        status: 0,
                    };
                    status_stack.push(new_status);
                }
                2 => {
                    status_stack.pop();
                    stack.pop();
                }
                _ => unreachable!()
            }

//            println!("This loop End: d = {:?}, stack = {:?}", status_stack.clone(), unsafe {
//                String::from_utf8_unchecked(stack.clone())
//            });
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn generate_parentheses_test() {
        assert_eq!(Solution::generate_parenthesis(3).iter().map(|s| &s[..]).collect::<Vec<&str>>(),
                   vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
        assert_eq!(Solution::generate_parenthesis(5).iter().map(|s| &s[..]).collect::<Vec<&str>>(),
                   vec!["((((()))))", "(((()())))", "(((())()))", "(((()))())", "(((())))()", "((()(())))", "((()()()))", "((()())())", "((()()))()", "((())(()))", "((())()())", "((())())()", "((()))(())", "((()))()()", "(()((())))", "(()(()()))", "(()(())())", "(()(()))()", "(()()(()))", "(()()()())", "(()()())()", "(()())(())", "(()())()()", "(())((()))", "(())(()())", "(())(())()", "(())()(())", "(())()()()", "()(((())))", "()((()()))", "()((())())", "()((()))()", "()(()(()))", "()(()()())", "()(()())()", "()(())(())", "()(())()()", "()()((()))", "()()(()())", "()()(())()", "()()()(())", "()()()()()"]);
        assert_eq!(Solution::generate_parenthesis(1).iter().map(|s| &s[..]).collect::<Vec<&str>>(),
                   vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(0).iter().map(|s| &s[..]).collect::<Vec<&str>>(),
                   vec![""]);
    }
}

