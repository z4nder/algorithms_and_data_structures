### O que é uma arvore ? 
Árvore, no contexto da programação, engenharia de software e ciência da computação, é uma das mais importantes estruturas de dados não *lineares* e justamente por essa definicão de uma estrutura não linear percebemos que estamos olhando pra uma estrutura com caracteristicas diferentes de outras vistas anteriormente como listas, em que os dados se encontram numa sequência, nas árvores os dados estão dispostos de forma hierárquica, seus elementos se encontram "acima" ou "abaixo" de outros elementos da árvore.

### Definicão formal de uma arvore
Formalmente, definimos uma árvore <T> como um conjunto finito de zero ou mais nodos tal que:
- se o número de nodos = 0 temos uma árvore vazia
- se o número de nodos > 0
    - existe um nó especialmente denominado raiz de <T>
    - Os nós restantes forma `m >= 0` conjuntos disjuntos `P1,P2...Pm`
    - Cada um desses nós restantes é uma arvore em si chamados de subarvores(assim como cada item de uma `linked` list é também uma lista)


### Algoritimo Árvore binária
A caracteristica que vai definir nossa arvore como binária é a seguinte:
Tem um elemento distinto, denominado raiz, com `dois ponteiros` para duas estruturas diferentes, denominadas subárvore esquerda e subárvore direita.