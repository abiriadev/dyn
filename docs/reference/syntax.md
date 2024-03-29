# Syntax Specification

## Conventions

1. Terminal symbols are written in $\text{roman}$ font.
2. Some special terminal symbols can be enclosed in $\text{'single quotes'}$, or can be represented as a unicode literal like $\text{U+2B7F}$.
3. Non-terminal syntactic symbols are written in $italic$ font.
4. Every non-terminal syntactic symbol names follow $PascaleCase$ naming rule.
5. Non-terminal lexemes are written in $\texttt{typewritter}$ font.
6. Every non-terminal lexeme names follow $\texttt{SCREAMING\_SNAKE\_CASE}$ naming rule.
7. $\texttt{EOF}$ is a special lexeme denoting the end of input.
8. $\epsilon$ denotes the empty string.
9. $A ~~ B$ means $A$ followed by $B$.
10. $A \mid B$ means either $A$ or $B$.
11. $A^n$ is a sequence of $n$ iterations of $A$.
12. $A^\ast$ is a sequence of zero or more iterations of $A$, which is equivalent to $P ::= \epsilon \mid A ~~ P$.
13. $A^+$ is a sequence of one or more iterations of $A$, which is equivalent to $AA^\ast$.
14. $A^?$ is an zero or one occurrence of $A$, which is equivalent to $\epsilon \mid A$.
15. ${\sim}A$ denotes any $Char$ except $A$. ($Char$ is defined [here](#characters))
16. $A ~~ [B]$ denotes positive lookahead of $B$, meaning $A$ followed by $B$.
17. $A ~~ {\sim}[B]$ denotes negative lookahead of $B$, meaning $A$ not followed by $B$.
18. $\langle A,\, B \rangle$ is a sequence of zero or more iterations of $A$ separated by $B$, ending with optional $B$, which is equivalent to $(A ~~ B)^\ast ~~ A^?$.
19. Production rules are written in the form of $A ::= B$.

## Lexical Structure

Note that Dyn's lexical structure has context-free grammar.

### Characters

$$
\begin{array}{}
Char & ::= & \text{U+0000} \mid \ldots \mid \text{U+D7FF} \mid \text{U+E000} \mid \ldots \mid \text{U+10FFFF} \\
\end{array}
$$

### Comments

$$
\begin{array}{lcl}
\texttt{LINE\_COMMENT} & ::= & \text{//} ~~ {\sim}(\text{U+000A} \mid \texttt{EOF})^\ast \\
\texttt{BLOCK\_COMMENT} & ::= & \text{'/*'} ~~ ({\sim}\text{'*'} \mid \text{'*'} ~~ {\sim}[\text{'/'}] \mid {\sim}\text{'/'} \mid \text{'/'} ~~ {\sim}[\text{'*'}] \mid \texttt{BLOCK\_COMMENT})^\ast ~~ \text{'*/'} \\
\end{array}
$$

### White Spaces

$$
\begin{array}{lcl}
\texttt{WHITESPACE} & ::= & \text{U+0009} \\
& \mid & \text{U+000B} \\
& \mid & \text{U+000D} \\
& \mid & \text{U+0020} \\
\end{array}
$$

### Nil

$$
\begin{array}{lcl}
\texttt{NIL} & ::= & \text{nil} \\
\end{array}
$$

### Boolean

$$
\begin{array}{lcl}
\texttt{BOOLEAN} & ::= & \text{true} \mid \text{false} \\
\end{array}
$$

### Number

There is no distinction between unsigned and signed integers.

$$
\begin{array}{lcl}
\texttt{INTEGER} & ::= & \texttt{DEC\_INTEGER} \\
& \mid & \texttt{BIN\_INTEGER} \\
& \mid & \texttt{OCT\_INTEGER} \\
& \mid & \texttt{HEX\_INTEGER} \\
\texttt{DEC\_INTEGER} & ::= & (\text{0} \mid \ldots \mid \text{9})^+ \\
\texttt{BIN\_INTEGER} & ::= & \text{0b} ~~ (\text{0} \mid \ldots \mid \text{1})^+ \\
\texttt{OCT\_INTEGER} & ::= & \text{0o} ~~ (\text{0} \mid \ldots \mid \text{7})^+ \\
\texttt{HEX\_INTEGER} & ::= & \text{0x} ~~ (\text{0} \mid \ldots \mid \text{9} \mid \text{a} \mid \ldots \mid \text{f} \mid \text{A} \ldots \mid \text{F})^+ \\
\texttt{FLOAT} & ::= & \texttt{DEC\_INTEGER} ~~ \text{'.'} ~~ {\sim}[\text{'.'}] \\
& \mid & \texttt{DEC\_INTEGER} ~~ \text{'.'} ~~ \texttt{DEC\_INTEGER} \\
& \mid & \text{'.'} ~~ \texttt{DEC\_INTEGER} \\
\end{array}
$$

### String

$$
\begin{array}{lcl}
\texttt{STRING} & ::= & \texttt{STRING\_SINGLE} \\
& \mid & \texttt{STRING\_DOUBLE} \\
\texttt{STRING\_SINGLE} & ::= & \text{'} ~~ \texttt{STRING\_SINGLE\_CHAR}^\ast ~~ \text{'} \\
\texttt{STRING\_DOUBLE} & ::= & \text{"} ~~ \texttt{STRING\_DOUBLE\_CHAR}^\ast ~~ \text{"} \\
\texttt{STRING\_SINGLE\_CHAR} & ::= & {\sim}(\text{\\} \mid \text{'}) \mid \text{\\} ~~ (\text{n} \mid \text{r} \mid \text{t} \mid \text{'} \mid \text{\\}) \\
\texttt{STRING\_DOUBLE\_CHAR} & ::= & {\sim}(\text{\\} \mid \text{"} \mid \text{\#}) \mid \text{'\#'} ~~ {\sim}[\text{'\{'}] \\
& \mid & \text{\\} ~~ (\text{n} \mid \text{r} \mid \text{t} \mid \text{"} \mid \text{\\} \mid \text{'\#\{'}) \\
\end{array}
$$

#### Template String

$$
\begin{array}{lcl}
\texttt{TEMPLATE\_STRING} & ::= & \texttt{TEMPLATE\_STRING\_HEAD} ~~ Expr ~~ (\texttt{TEMPLATE\_STRING\_BODY} ~~ Expr)^\ast ~~ \texttt{TEMPLATE\_STRING\_TAIL} \\
\texttt{TEMPLATE\_STRING\_HEAD} & ::= & \text{"} ~~ \texttt{STRING\_DOUBLE\_CHAR}^\ast ~~ \text{'\#\{'} \\
\texttt{TEMPLATE\_STRING\_BODY} & ::= & \text{'\}'} ~~ \texttt{STRING\_DOUBLE\_CHAR}^\ast ~~ \text{'\#\{'} \\
\texttt{TEMPLATE\_STRING\_TAIL} & ::= & \text{'\}'} ~~ \texttt{STRING\_DOUBLE\_CHAR}^\ast ~~ \text{"} \\
\end{array}
$$

### Identifiers

$\texttt{XID\_START}$ and $\texttt{XID\_CONTINUE}$ are defined in Unicode Standard Annex #31.[^1]

$$
\begin{array}{lcl}
\texttt{IDENT\_OR\_KEYWORD} & ::= \texttt{XID\_START} ~~ \texttt{XID\_CONTINUE}^\ast \\
\end{array}
$$

The below are every keyword in Dyn. Reserved keywrods are NOT used yet, but they are reserved for future use and treated as just normal keywords.

| Lexeme                  | Symbol            | Reserved |
| :---------------------- | :---------------- | -------- |
| $\texttt{KW\_BREAK}$    | $\text{break}$    | X        |
| $\texttt{KW\_CONTINUE}$ | $\text{continue}$ | X        |
| $\texttt{KW\_ELSE}$     | $\text{else}$     | X        |
| $\texttt{KW\_EXPORT}$   | $\text{export}$   | O        |
| $\texttt{KW\_FALSE}$    | $\text{false}$    | X        |
| $\texttt{KW\_IF}$       | $\text{if}$       | X        |
| $\texttt{KW\_ITER}$     | $\text{iter}$     | X        |
| $\texttt{KW\_LET}$      | $\text{let}$      | X        |
| $\texttt{KW\_NIL}$      | $\text{nil}$      | X        |
| $\texttt{KW\_OF}$       | $\text{of}$       | X        |
| $\texttt{KW\_RETURN}$   | $\text{return}$   | X        |
| $\texttt{KW\_TRUE}$     | $\text{true}$     | X        |
| $\texttt{KW\_USE}$      | $\text{use}$      | O        |

### Punctuations

| Lexeme                             | Symbol        | Reserved |
| :--------------------------------- | :------------ | -------- |
| $\texttt{RightBrace}$              | $\text{\}}$   | X        |
| $\texttt{PipePipe}$                | $\text{\|\|}$ | X        |
| $\texttt{Pipe}$                    | $\text{\|}$   | X        |
| $\texttt{LeftBrace}$               | $\text{\{}$   | X        |
| $\texttt{RightBracket}$            | $\text{]}$    | X        |
| $\texttt{NewLine}$                 | $\text{\\n}$  | X        |
| $\texttt{LeftBracket}$             | $\text{[}$    | X        |
| $\texttt{At}$                      | $\text{@}$    | O        |
| $\texttt{RightAngledBracketEqual}$ | $\text{>=}$   | X        |
| $\texttt{RightAngledBracket}$      | $\text{>}$    | X        |
| $\texttt{EqualEqual}$              | $\text{==}$   | X        |
| $\texttt{Equal}$                   | $\text{=}$    | X        |
| $\texttt{LeftAngledBracketEqual}$  | $\text{<=}$   | X        |
| $\texttt{LeftAngledBracket}$       | $\text{<}$    | X        |
| $\texttt{Colon}$                   | $\text{:}$    | X        |
| $\texttt{SlashEqual}$              | $\text{/=}$   | X        |
| $\texttt{Slash}$                   | $\text{/}$    | X        |
| $\texttt{DotDot}$                  | $\text{..}$   | X        |
| $\texttt{Dot}$                     | $\text{.}$    | X        |
| $\texttt{Arrow}$                   | $\text{->}$   | X        |
| $\texttt{MinusEqual}$              | $\text{-=}$   | X        |
| $\texttt{Minus}$                   | $\text{-}$    | X        |
| $\texttt{Comma}$                   | $\text{,}$    | X        |
| $\texttt{PlusEqual}$               | $\text{+=}$   | X        |
| $\texttt{Plus}$                    | $\text{+}$    | X        |
| $\texttt{AsteriskEqual}$           | $\text{*=}$   | X        |
| $\texttt{Asterisk}$                | $\text{*}$    | X        |
| $\texttt{RightParenthesis}$        | $\text{)}$    | X        |
| $\texttt{LeftParenthesis}$         | $\text{(}$    | X        |
| $\texttt{AndAnd}$                  | $\text{\&\&}$ | X        |
| $\texttt{PercentEqual}$            | $\text{\%=}$  | X        |
| $\texttt{Percent}$                 | $\text{\%}$   | X        |
| $\texttt{BangEqual}$               | $\text{!=}$   | X        |
| $\texttt{Bang}$                    | $\text{!}$    | X        |

## Syntectic Structure

### Literals

$$
\begin{array}{lcl}
Literals & ::= & NilLiteral \\
& \mid & BooleanLiteral \\
& \mid & IntegerLiteral \\
& \mid & FloatLiteral \\
& \mid & StringLiteral \\
& \mid & ArrayLiteral \\
& \mid & RecordLiteral \\
& \mid & FunctionLiteral \\
NilLiteral & ::= & \texttt{NIL} \\
BooleanLiteral & ::= & \texttt{BOOLEAN} \\
IntegerLiteral & ::= & \texttt{INTEGER} \\
FloatLiteral & ::= & \texttt{FLOAT} \\
StringLiteral & ::= & \texttt{STRING} \\
ArrayLiteral & ::= & \text{[} ~~ \langle Expr,\, \text{','} \rangle ~~ \text{]} \\
RecordLiteral & ::= & \text{'('} ~~ \langle Ident ~~ \text{':'} ~~ Expr,\, \text{','} \rangle ~~ \text{')'} \\
\end{array}
$$

#### FunctionLiteral

$$
\begin{array}{lcl}
FunctionParameters & ::= & \langle Ident,\, \text{','} \rangle \\
FunctionLiteral & ::= & ( ~~ \text{'('} ~~ FunctionParameters ~~ \text{')'} ~~ )^? ~~ \text{->} ~~ Expr \\
\end{array}
$$

---

[^1]: https://www.unicode.org/reports/tr31/
