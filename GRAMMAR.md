# HILO Grammar (EBNF)

This grammar is intentionally compact and unambiguous. Newlines terminate most statements.
A semicolon `;` is optional when the newline is present.

```
Program        = ModuleDecl? Import* TopLevel* ;

ModuleDecl     = "module" QName ;
Import         = "import" ImportPath ( "as" IDENT )? ( "{" IdentList "}" )? ;
ImportPath     = QName ;
IdentList      = IDENT ( "," IDENT )* ;
QName          = IDENT ( "." IDENT )* ;

TopLevel       = Declaration | TaskDecl | WorkflowDecl | AgentDecl | TestDecl ;

Declaration    = ConstDecl | VarDecl | LetDecl | TypeDecl | RecordDecl | EnumDecl
                 | TraitDecl | ClassDecl | FuncDecl | ExportDecl ;

ExportDecl     = "export" ( FuncDecl | RecordDecl | EnumDecl | ClassDecl | TraitDecl | TypeDecl ) ;

ConstDecl      = "const" IDENT ":" Type "=" Expr ;
LetDecl        = "let" IDENT ( ":" Type )? ( "=" Expr )? ;
VarDecl        = "var" IDENT ( ":" Type )? ( "=" Expr )? ;

TypeDecl       = "type" IDENT TypeParams? "=" Type ;
RecordDecl     = "record" IDENT TypeParams? "{" FieldDecl* "}" ;
FieldDecl      = IDENT ":" Type ( "=" Expr )? ;

EnumDecl       = "enum" IDENT TypeParams? "{" EnumCase ("," EnumCase)* "}" ;
EnumCase       = IDENT TypeArgs? ( "(" ParamList? ")" )? ;

TraitDecl      = "trait" IDENT TypeParams? "{" TraitMember* "}" ;
TraitMember    = FuncSig ";" ;

ClassDecl      = "class" IDENT TypeParams? ( "implements" QName ( "," QName )* )? "{" ClassMember* "}" ;
ClassMember    = FieldDecl | FuncDecl | CtorDecl | PropDecl ;
CtorDecl       = "new" "(" ParamList? ")" Block ;
PropDecl       = "prop" IDENT ":" Type ( "get" Block )? ( "set" Block )? ;

FuncDecl       = "func" IDENT TypeParams? "(" ParamList? ")" ( "->" Type )? Block ;
FuncSig        = "func" IDENT TypeParams? "(" ParamList? ")" ( "->" Type )? ;
ParamList      = Param ( "," Param )* ;
Param          = IDENT ":" Type ( "=" Expr )? ;

TypeParams     = "<" IDENT ( "," IDENT )* ">" ;
TypeArgs       = "<" Type ( "," Type )* ">" ;

AgentDecl      = "agent" IDENT "{" AgentMember* "}" ;
AgentMember    = "profile" Block
               | "capabilities" Block
               | "tools" Block
               | "policy" Block
               | FuncDecl ;

TaskDecl       = "task" IDENT "(" ParamList? ")" ( "->" Type )? Block ;

WorkflowDecl   = "workflow" IDENT Block ;

TestDecl       = "test" ( STRING | IDENT ) Block ;

Block          = "{" Stmt* "}" ;

Stmt           = SimpleStmt
               | IfStmt | WhileStmt | ForStmt | MatchStmt
               | TryStmt | UsingStmt | DeferStmt
               | ReturnStmt | BreakStmt | ContinueStmt
               | SpawnStmt | ChannelStmt | SendStmt | RecvStmt | SelectStmt ;

SimpleStmt     = VarDecl | LetDecl | ExprStmt ;

IfStmt         = "if" Expr Block ( "else" ( IfStmt | Block ) )? ;
WhileStmt      = "while" Expr Block ;
ForStmt        = "for" IDENT "in" Expr Block ;

MatchStmt      = "match" Expr "{" CaseClause+ "}" ;
CaseClause     = Pattern "=>" ( Expr | Block ) ;

TryStmt        = "try" Block "catch" "(" IDENT ")" Block ;
UsingStmt      = "using" "(" Expr ")" Block ;
DeferStmt      = "defer" Block ;

ReturnStmt     = "return" Expr? ;
BreakStmt      = "break" ;
ContinueStmt   = "continue" ;

SpawnStmt      = "spawn" Expr ;
ChannelStmt    = "channel" "<" Type ">" "(" ( "capacity" "=" INT )? ")" ;
SendStmt       = "send" Expr "<-" Expr ;
RecvStmt       = "recv" Expr "->" IDENT ;
SelectStmt     = "select" "{" SelectCase+ ( "timeout" Duration "=>" Block )? "}" ;
SelectCase     = ( "send" Expr "<-" Expr | "recv" Expr "->" IDENT ) "=>" Block ;

ExprStmt       = Expr ;

Expr           = Assign ;
Assign         = Or ( "=" Or )? ;
Or             = And ( "or" And )* ;
And            = Cmp ( "and" Cmp )* ;
Cmp            = Add ( ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) Add )* ;
Add            = Mul ( ( "+" | "-" ) Mul )* ;
Mul            = Unary ( ( "*" | "/" | "%" ) Unary )* ;
Unary          = ( "-" | "not" | "await" ) Unary | Postfix ;
Postfix        = Primary ( Call | Index | Field | OptChain | Pipe )* ;
Call           = "(" ArgList? ")" ;
ArgList        = Arg ( "," Arg )* ;
Arg            = ( IDENT ":" )? Expr ;
Index          = "[" Expr "]" ;
Field          = "." IDENT ;
OptChain       = "?." IDENT ;
Pipe           = "|>" Primary ;   // simple, left-associative

Primary        = INT | FLOAT | STRING | "true" | "false" | "null"
               | IDENT
               | "(" Expr ")"
               | Lambda
               | ListLit | MapLit | TupleLit ;

Lambda         = "fn" "(" ParamList? ")" ( "->" Type )? Block ;
ListLit        = "[" ( Expr ( "," Expr )* )? "]" ;
MapLit         = "map" "{" ( (Expr ":" Expr) ( "," Expr ":" Expr )* )? "}" ;
TupleLit       = "(" Expr "," Expr ( "," Expr )* ")" ;

Pattern        = "_" | IDENT | Literal | RecordPat | EnumPat | TuplePat ;
RecordPat      = IDENT "{" ( IDENT ( ":" Pattern )? ( "," IDENT ( ":" Pattern )? )* )? "}" ;
EnumPat        = IDENT ( "." IDENT )? ( "(" Pattern ( "," Pattern )* ")" )? ;
TuplePat       = "(" Pattern "," Pattern ( "," Pattern )* ")" ;

Type           = QName TypeArgs?
               | ListType | MapType | TupleType
               | FuncType ;

ListType       = "List" "[" Type "]" | "[" Type "]" ;
MapType        = "Map" "[" Type "," Type "]" ;
TupleType      = "Tuple" "[" Type ( "," Type )+ "]" ;
FuncType       = "(" ( Type ( "," Type )* )? ")" "->" Type ;

IDENT          = /[A-Za-z_][A-Za-z0-9_]*/ ;
INT            = /-?[0-9]+/ ;
FLOAT          = /-?[0-9]+\\.[0-9]+/ ;
STRING         = /"([^"\\\\]|\\\\.)*"/ ;
Duration       = /"[0-9]+(ms|s|m|h|d)"/ ;
```

Notes:
- Blocks use `{ ... }` only (no indentation sensitivity).
- Newlines end statements; semicolons are optional.
- This grammar is intentionally minimal; see `LANGUAGE_SPEC.md` for semantics.
