# Benchmarks e Fuzzing - Xavier XML

Este documento descreve como executar benchmarks e fuzzing no projeto Xavier XML.

## 🚀 Benchmarks

### Configuração

Os benchmarks estão configurados usando [Criterion.rs](https://bheisler.github.io/criterion.rs/).

#### Dependências

```toml
[dev-dependencies]
criterion = "0.5"
```

#### Estrutura

```
benches/
├── benchmarks.rs          # Benchmarks principais
└── criterion.toml         # Configuração do Criterion
```

### Executando Benchmarks

#### Método 1: Script Automatizado

```bash
./scripts/run_benchmarks.sh
```

#### Método 2: Comando Manual

```bash
# Executar todos os benchmarks
cargo bench

# Executar benchmark específico
cargo bench --bench benchmarks -- serialize_simple_struct

# Executar com perfil rápido
cargo bench --profile quick
```

### Benchmarks Disponíveis

1. **serialize_simple_struct** - Serialização de struct simples
2. **deserialize_simple_struct** - Deserialização de struct simples
3. **serialize_complex_struct** - Serialização de struct complexa
4. **deserialize_complex_struct** - Deserialização de struct complexa
5. **serialize_large_struct** - Serialização de struct grande
6. **serialize_large_dataset** - Serialização de dataset grande
7. **serialize_special_characters** - Serialização com caracteres especiais
8. **deserialize_special_characters** - Deserialização com caracteres especiais
9. **serialize_memory_intensive** - Serialização intensiva de memória

### Resultados

Os resultados são salvos em:
- `target/criterion/` - Relatórios detalhados
- `target/criterion/report/` - Relatórios HTML

## 🔍 Fuzzing

### Configuração

O fuzzing está configurado usando [cargo-fuzz](https://rust-fuzz.github.io/book/).

#### Estrutura

```
fuzz/
├── Cargo.toml                    # Configuração do fuzzing
├── fuzz_targets/
│   ├── xml_parsing.rs           # Fuzzing de parsing
│   ├── xml_serialization.rs     # Fuzzing de serialização
│   └── xml_edge_cases.rs        # Fuzzing de casos edge
```

### Executando Fuzzing

#### Método 1: Script Automatizado

```bash
./scripts/run_fuzzing.sh
```

#### Método 2: Comando Manual

```bash
# Instalar cargo-fuzz (se necessário)
cargo install cargo-fuzz

# Entrar no diretório de fuzzing
cd fuzz

# Executar fuzzing para parsing
cargo fuzz run xml_parsing

# Executar fuzzing para serialização
cargo fuzz run xml_serialization

# Executar fuzzing para casos edge
cargo fuzz run xml_edge_cases
```

### Targets de Fuzzing

1. **xml_parsing** - Testa parsing de XML malformado
2. **xml_serialization** - Testa round-trip de serialização
3. **xml_edge_cases** - Testa casos edge e limites

### Configurações de Fuzzing

- **Tamanho máximo**: 10.000 bytes
- **Timeout**: 30 segundos
- **Corpus**: Gerado automaticamente

### Resultados

Os resultados são salvos em:
- `fuzz/artifacts/` - Casos de teste que causaram falhas
- `fuzz/corpus/` - Corpus de entrada

## 📊 Métricas e Análise

### Benchmarks

#### Métricas Coletadas

- **Tempo de execução** - Latência média e percentis
- **Throughput** - Operações por segundo
- **Uso de memória** - Alocações e tamanho
- **Variação** - Estabilidade dos resultados

#### Interpretação

- **Melhor** - Menor tempo de execução
- **Pior** - Maior tempo de execução
- **Estável** - Baixa variação entre execuções

### Fuzzing

#### Métricas Coletadas

- **Cobertura** - Cobertura de código alcançada
- **Crashes** - Falhas encontradas
- **Timeouts** - Timeouts detectados
- **Corpus** - Tamanho do corpus

#### Interpretação

- **Cobertura alta** - Boa cobertura de casos de teste
- **Crashes baixos** - Poucas falhas encontradas
- **Timeouts baixos** - Performance estável

## 🛠️ Desenvolvimento

### Adicionando Novos Benchmarks

1. Editar `benches/benchmarks.rs`
2. Adicionar nova função de benchmark
3. Registrar no `criterion_group!`
4. Executar `cargo bench`

### Adicionando Novos Targets de Fuzzing

1. Criar novo arquivo em `fuzz/fuzz_targets/`
2. Adicionar `[[bin]]` no `fuzz/Cargo.toml`
3. Implementar lógica de fuzzing
4. Executar `cargo fuzz run <target>`

### Configurações Avançadas

#### Criterion

```toml
[profiles.custom]
confidence-level = 0.99
measurement-time = 10
nresamples = 200000
noise-threshold = 0.01
```

#### Fuzzing

```bash
# Executar com configurações customizadas
cargo fuzz run xml_parsing -- -max_len=50000 -timeout=60 -runs=1000000
```

## 📈 Monitoramento Contínuo

### CI/CD

Os benchmarks e fuzzing podem ser integrados ao CI/CD:

```yaml
# GitHub Actions example
- name: Run Benchmarks
  run: cargo bench

- name: Run Fuzzing
  run: |
    cargo install cargo-fuzz
    cd fuzz
    cargo fuzz run xml_parsing -- -max_len=1000 -timeout=10
```

### Alertas

- **Regressão de performance** - Benchmarks pioraram
- **Novos crashes** - Fuzzing encontrou falhas
- **Cobertura baixa** - Fuzzing não está cobrindo código

## 🔧 Troubleshooting

### Problemas Comuns

#### Benchmarks

**Erro: "no benchmark target found"**
```bash
# Verificar se o target está registrado
cargo bench --list
```

**Erro: "criterion not found"**
```bash
# Instalar criterion
cargo install criterion
```

#### Fuzzing

**Erro: "cargo-fuzz not found"**
```bash
# Instalar cargo-fuzz
cargo install cargo-fuzz
```

**Erro: "target not found"**
```bash
# Verificar se o target está registrado
cargo fuzz list
```

### Performance

#### Otimizações de Benchmark

1. **Reduzir noise** - Aumentar `measurement-time`
2. **Melhorar precisão** - Aumentar `nresamples`
3. **Filtrar outliers** - Ajustar `noise-threshold`

#### Otimizações de Fuzzing

1. **Aumentar cobertura** - Expandir corpus
2. **Reduzir timeouts** - Otimizar código
3. **Melhorar seed** - Usar corpus inicial

## 📚 Recursos

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/)
- [Cargo Fuzz Book](https://rust-fuzz.github.io/book/)
- [Rust Fuzz Targets](https://github.com/rust-fuzz/targets)

---

**Última atualização**: $(date)  
**Versão**: 1.0  
**Responsável**: Equipe de Desenvolvimento Xavier XML 