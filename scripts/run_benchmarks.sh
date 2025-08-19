#!/bin/bash

# Script para executar benchmarks do Xavier XML
set -e

echo "🚀 Executando benchmarks do Xavier XML..."

# Verificar se estamos no diretório correto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Erro: Execute este script na raiz do projeto"
    exit 1
fi

# Instalar criterion se não estiver instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Erro: Cargo não encontrado"
    exit 1
fi

# Executar benchmarks
echo "📊 Executando benchmarks..."
cargo bench

echo "✅ Benchmarks concluídos!"
echo "📁 Resultados disponíveis em: target/criterion/" 