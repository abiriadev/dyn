# Syntax Specification

## Conventions

1. Terminal symbols are written in $\text{roman}$ font.
2. Some special terminal symbols can be enclosed in $\text{'single quotes'}$, or can be represented as a unicode literal like $\text{U+2B7F}$.
3. Non-terminal syntactic symbols are written in $italic$ font.
4. Every non-terminal syntactic symbol names follow $PascaleCase$ naming rule.
5. Non-terminal lexemes are written in $\texttt{typewritter}$ font.
6. Every non-terminal lexeme names follow $\texttt{SCREAMING\_SNAKE\_CASE}$ naming rule.
7. $\epsilon$ denotes the empty string.
8. $A ~~ B$ means $A$ followed by $B$.
9. $A \mid B$ means either $A$ or $B$.
10. $A^n$ is a sequence of $n$ iterations of $A$.
11. $A^\ast$ is a sequence of zero or more iterations of $A$, which is equivalent to $P ::= \epsilon \mid A ~~ P$.
12. $A^+$ is a sequence of one or more iterations of $A$, which is equivalent to $AA^\ast$.
13. $A^?$ is an zero or one occurrence of $A$, which is equivalent to $\epsilon \mid A$.
14. ${\sim}A$ denotes any $Char$ except $A$. ($Char$ is defined [here](#characters))
15. $\langle A,\, B \rangle$ is a sequence of zero or more iterations of $A$ separated by $B$, ending with optional $B$, which is equivalent to $(A ~~ B)^\ast ~~ A^?$.
16. Production rules are written in the form of $A ::= B$.

## Lexical Structure

Note that Dyn's lexical structure has context-free grammar.

### Characters

$$
\begin{array}{}
Char & ::= & \text{U+0000} \mid \ldots \mid \text{U+D7FF} \mid \text{U+E000} \mid \ldots \mid \text{U+10FFFF} \\
\end{array}
$$

### Comments

## Syntectic Structure

### Literals

$$
\begin{array}{lcl}
Literals & ::= & NilLiteral \\
& \mid & BooleanLiteral \\
& \mid & NumberLiteral \\
& \mid & StringLiteral \\
& \mid & ArrayLiteral \\
& \mid & RecordLiteral \\
& \mid & FunctionLiteral \\
\end{array}
$$

#### NilLiteral

$$
\begin{array}{lcl}
NilLiteral & ::= & NilToken \\
\end{array}
$$

#### BooleanLiteral

$$
\begin{array}{lcl}
NilLiteral & ::= & BooleanToken \\
\end{array}
$$

#### NumberLiteral

$$
\begin{array}{lcl}
NilLiteral & ::= & NumberToken \\
\end{array}
$$

#### StringLiteral

$$
\begin{array}{lcl}
NilLiteral & ::= & StringToken \\
\end{array}
$$

#### ArrayLiteral

$$
\begin{array}{lcl}
ArrayLiteral & ::= & \text{[} ~~ \langle Expr,\, \text{','} \rangle ~~ \text{]} \\
\end{array}
$$

#### RecordLiteral

$$
\begin{array}{lcl}
RecordLiteral ::= & \text{'('} ~~ \langle Ident ~~ \text{':'} ~~ Expr,\, \text{','} \rangle ~~ \text{')'} \\
\end{array}
$$

#### FunctionLiteral

$$
\begin{array}{lcl}
FunctionParameters & ::= & \langle Ident,\, \text{','} \rangle \\
FunctionLiteral & ::= & ( ~~ \text{'('} ~~ FunctionParameters ~~ \text{')'} ~~ )^? ~~ \text{->} ~~ Expr \\
\end{array}
$$
