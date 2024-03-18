# Syntax Specification

## Conventions

1. Terminal symbols are written in $\text{roman}$ font.
2. Some special terminal symbols can be enclosed in $\text{'single quotes'}$, or can be represented as a unicode literal like $\text{U+2B7F}$.
3. Non-terminal symbols are written in $italic$ font.
4. Every non-terminal symbol names follow $PascaleCase$ naming rule.
5. $\epsilon$ denotes the empty string.
6. $A ~~ B$ means $A$ followed by $B$.
7. $A \mid B$ means either $A$ or $B$.
8. $A^n$ is a sequence of $n$ iterations of $A$.
9. $A^\ast$ is a sequence of zero or more iterations of $A$, which is equivalent to $P ::= \epsilon \mid A ~~ P$.
10. $A^+$ is a sequence of one or more iterations of $A$, which is equivalent to $AA^\ast$.
11. $A^?$ is an zero or one occurrence of $A$, which is equivalent to $\epsilon \mid A$.
12. Production rules are written in the form of $A ::= B$.

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
ArrayLiteral & ::= & \text{[} ~~ Punctuated \langle Expr \rangle ~~ \text{]} \\
\end{array}
$$

#### RecordLiteral

$$
\begin{array}{lcl}
ArrayLiteral & ::= & \text{'('} ~~ Punctuated \langle Ident ~~ \text{':'} ~~ Expr \rangle ~~ \text{')'} \\
\end{array}
$$
