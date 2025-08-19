#!/bin/bash

# Script para executar fuzzing do Xavier XML
set -e

echo "🔍 Executando fuzzing do Xavier XML..."

# Verificar se estamos no diretório correto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Erro: Execute este script na raiz do projeto"
    exit 1
fi

# Verificar se cargo-fuzz está instalado
if ! command -v cargo-fuzz &> /dev/null; then
    echo "📦 Instalando cargo-fuzz..."
    cargo install cargo-fuzz
fi

# Entrar no diretório de fuzzing
cd fuzz

# Executar fuzzing para cada target
echo "🎯 Executando fuzzing para xml_parsing..."
cargo fuzz run xml_parsing -- -max_len=10000 -timeout=30 &

echo "🎯 Executando fuzzing para xml_serialization..."
cargo fuzz run xml_serialization -- -max_len=10000 -timeout=30 &

echo "🎯 Executando fuzzing para xml_edge_cases..."
cargo fuzz run xml_edge_cases -- -max_len=10000 -timeout=30 &

# Aguardar todos os processos terminarem
wait

echo "✅ Fuzzing concluído!"
echo "📁 Resultados disponíveis em: fuzz/artifacts/" 