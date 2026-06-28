'use client'
import { useState, useEffect } from 'react'
export default function Home() {
  const [languages, setLanguages] = useState<any[]>([])
  const [selected, setSelected] = useState<any>(null)
  useEffect(() => { fetch('/api/languages').then(r => r.json()).then(setLanguages) }, [])
  const open = async (slug: string) => {
    const res = await fetch(`/api/languages/${slug}`)
    const data = await res.json()
    setSelected(data)
  }
  return (
    <main className="min-h-screen p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-5xl font-bold mb-2 bg-gradient-to-r from-green-400 to-blue-600 bg-clip-text text-transparent">LearnXInMinutes</h1>
        <p className="text-gray-400 mb-8">Any language, fast</p>
        {selected ? (
          <div>
            <button onClick={() => setSelected(null)} className="mb-4 text-green-400 hover:text-green-300">← Back</button>
            <h2 className="text-3xl font-bold mb-4">{selected.icon} {selected.name}</h2>
            <pre className="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20 overflow-x-auto font-mono text-sm whitespace-pre-wrap">{selected.content}</pre>
          </div>
        ) : (
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {languages.map((lang: any) => (
              <div key={lang.slug} onClick={() => open(lang.slug)} className="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20 text-center cursor-pointer hover:scale-105 transition">
                <div className="text-4xl mb-2">{lang.icon}</div>
                <h3 className="font-semibold">{lang.name}</h3>
                <span className="text-xs px-2 py-1 bg-green-500/20 text-green-300 rounded-full">{lang.category}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </main>
  )
}
