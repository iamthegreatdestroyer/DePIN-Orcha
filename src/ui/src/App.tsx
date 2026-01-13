// DePIN Orcha UI - App.tsx
import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="min-h-screen bg-slate-900 text-white">
      <header className="bg-slate-800 border-b border-slate-700">
        <div className="container mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold">DePIN Orcha</h1>
          <p className="text-slate-400">Intelligent Multi-Protocol DePIN Orchestrator</p>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          {/* Placeholder cards for future components */}
          <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
            <h3 className="text-xl font-semibold mb-2">Coming Soon</h3>
            <p className="text-slate-400">Dashboard features will be displayed here</p>
          </div>
        </div>
      </main>
    </div>
  )
}

export default App
