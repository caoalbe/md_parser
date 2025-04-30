use md_parser::ast::{Tree, run_ast};
use md_parser::lexer::{Token, TokenType::*};

#[test]
fn literal_one() {
    let token_vec: Vec<Token> = vec![Token {
        token_type: Literal,
        value: String::from("text1"),
    }];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <p>text1</p>
</html>"
    )
}

#[test]
fn literal_two() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Literal,
            value: String::from("text1"),
        },
        Token {
            token_type: Literal,
            value: String::from("text2"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <p>text1</p>
    <p>text2</p>
</html>"
    )
}

#[test]
fn header_prefix() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Prefix,
            value: String::from("h1"),
        },
        Token {
            token_type: Literal,
            value: String::from("text2"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <h1>text2</h1>
</html>"
    )
}

#[test]
fn header_suffix() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Literal,
            value: String::from("text2"),
        },
        Token {
            token_type: Suffix,
            value: String::from("h1"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <h1>text2</h1>
</html>"
    )
}

#[test]
fn table_header() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Literal,
            value: String::from("Header 1|Header 2|Header 3"),
        },
        Token {
            token_type: Suffix,
            value: String::from("table"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <table>
        <tr>
            <th>Header 1</th>
            <th>Header 2</th>
            <th>Header 3</th>
        </tr>
    </table>
</html>"
    )
}

#[test]
fn table_header2() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Literal,
            value: String::from("Header1 |"),
        },
        Token {
            token_type: Suffix,
            value: String::from("table"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <table>
        <tr>
            <th>Header1 </th>
        </tr>
    </table>
</html>"
    )
}

#[test]
fn table_header3() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Literal,
            value: String::from("|Header1"),
        },
        Token {
            token_type: Suffix,
            value: String::from("table"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <table>
        <tr>
            <th>Header1</th>
        </tr>
    </table>
</html>"
    )
}

#[test]
fn table() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Literal,
            value: String::from("Header 1|Header 2|Header 3"),
        },
        Token {
            token_type: Suffix,
            value: String::from("table"),
        },
        Token {
            token_type: Literal,
            value: String::from("1,1|1,2|1,3"),
        },
        Token {
            token_type: Literal,
            value: String::from("2,1|2,2|"),
        },
        Token {
            token_type: Literal,
            value: String::from("|3,1|3,2|3,3|3,4|"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <table>
        <tr>
            <th>Header 1</th>
            <th>Header 2</th>
            <th>Header 3</th>
        </tr>
        <tr>
            <td>1,1</td>
            <td>1,2</td>
            <td>1,3</td>
        </tr>
        <tr>
            <td>2,1</td>
            <td>2,2</td>
        </tr>
        <tr>
            <td>3,1</td>
            <td>3,2</td>
            <td>3,3</td>
            <td>3,4</td>
        </tr>
    </table>
</html>"
    )
}

#[test]
fn combination() {
    let token_vec: Vec<Token> = vec![
        Token {
            token_type: Prefix,
            value: String::from("h2"),
        },
        Token {
            token_type: Literal,
            value: String::from("Header"),
        },
        Token {
            token_type: Suffix,
            value: String::from("empty_line"),
        },
        Token {
            token_type: Literal,
            value: String::from("Donec non massa quis est blandit volutpat. Donec sit amet."),
        },
        Token {
            token_type: Literal,
            value: String::from("Header 1|Header 2|Header 3"),
        },
        Token {
            token_type: Suffix,
            value: String::from("table"),
        },
        Token {
            token_type: Literal,
            value: String::from("1,1|1,2|1,3"),
        },
        Token {
            token_type: Literal,
            value: String::from("2,1|2,2|"),
        },
        Token {
            token_type: Literal,
            value: String::from("|3,1|3,2|3,3|3,4|"),
        },
        Token {
            token_type: Suffix,
            value: String::from("empty_line"),
        },
        Token {
            token_type: Literal,
            value: String::from("Nam vitae felis lectus. Sed sodales faucibus erat, a porttitor."),
        },
        Token {
            token_type: Prefix,
            value: String::from("h3"),
        },
        Token {
            token_type: Literal,
            value: String::from("Header 3"),
        },
    ];
    let output: Tree = run_ast(token_vec);
    assert_eq!(
        output.to_string().as_str(),
        "<html>
    <h2>Header</h2>
    <p>Donec non massa quis est blandit volutpat. Donec sit amet.</p>
    <table>
        <tr>
            <th>Header 1</th>
            <th>Header 2</th>
            <th>Header 3</th>
        </tr>
        <tr>
            <td>1,1</td>
            <td>1,2</td>
            <td>1,3</td>
        </tr>
        <tr>
            <td>2,1</td>
            <td>2,2</td>
        </tr>
        <tr>
            <td>3,1</td>
            <td>3,2</td>
            <td>3,3</td>
            <td>3,4</td>
        </tr>
    </table>
    <p>Nam vitae felis lectus. Sed sodales faucibus erat, a porttitor.</p>
    <h3>Header 3</h3>
</html>"
    )
}
