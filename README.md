# Wizard API

Uma minúscula API REST para o acompanhamento das etapas no ciclo de vida de qualquer coisa
com múltiplos estágios.

Esta micro-API foi idealizada para servir a função de acompanhar o ciclo de vida de *contratos* que
passam por múltiplos passos, contratos que possuem uma sequência de estados possíveis, como
num "[wizard](https://pt.wikipedia.org/wiki/Assistente_(software))" de instalação de software.

## Visão Geral

A contratação de um serviço, ou a compra de um produto online pode ser um processo curto ou extenso
dependendo da coisa a ser contratada ou comprada.

Por exemplo, um curso online de 4 semanas pode ter 4 formulários com testes para cada aluno
completar antes de receber uma certifição. O acompanhamento de em qual "semana" de um curso cada
aluno da sala está, ou se ele cancelou a matrícula poderia usar esta micro-API para auxiliar
na gerencia deste processo que possui múltiplos estágios.

A representação deste contrato, em JSON pode ser imaginada como algo parecido com:

```json
{
  "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "created_by": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "created_at": "2022-12-06T19:13:47.894Z",
  "updated_at": "2022-12-06T19:13:47.894Z",
  "status": "active/week-3",
  "setup_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```

Esta API não possui a finalidade de ser abrangente, mas sim ser focada, genérica flexível e simples.
Para ser usada como um microsserviço em conjunto com outras APIs pequenas.

A Wizard API registra a mudança de estados de um contrato e as referências aos componentes deste
contrato, a criação destes componentes está fora do escopo deste projeto.

## Licença

MIT

## Referência

Consulte a especificação [OpenAPI do projeto](./spec/openapi.yaml) ([ReDoc](https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/jaxyendy/wizard-api/main/spec/openapi.yaml), [Swagger](https://petstore.swagger.io/?url=https%3A%2F%2Fraw.githubusercontent.com%2Fjaxyendy%2Fwizard-api%2Fmain%2Fspec%2Fopenapi.yaml#/))
