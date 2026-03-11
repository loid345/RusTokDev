# rustok-forum docs

В этой папке хранится документация модуля `crates/rustok-forum`.

## Documents

- [Implementation plan](./implementation-plan.md)

## Event contracts

- [Event flow contract (central)](../../../docs/architecture/event-flow-contract.md)

## Rich-text contract

- Blog/Forum backend принимает `markdown` и `rt_json_v1` payload; для `rt_json_v1` выполняется обязательные `schema validation + sanitize` на сервере.
- Клиентские валидаторы считаются advisory-only и не являются источником истины.
- Детали спецификации: [docs/standards/rt-json-v1.md](../../../docs/standards/rt-json-v1.md).
