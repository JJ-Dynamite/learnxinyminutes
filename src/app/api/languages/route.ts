import { NextResponse } from 'next/server'
export async function GET() {
  const res = await fetch('http://localhost:3001/languages')
  return NextResponse.json(await res.json())
}
