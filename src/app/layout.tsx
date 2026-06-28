import './globals.css'
export const metadata = { title: 'LearnXInMinutes - Any language, fast', description: 'Any language, fast' }
export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (<html lang="en"><body className="antialiased">{children}</body></html>)
}
