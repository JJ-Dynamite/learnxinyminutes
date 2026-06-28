import { NextRequest, NextResponse } from 'next/server'
export async function GET(req: NextRequest, { params }: { params: { name: string } }) {
  const res = await fetch(`http://localhost:3001/languages/${params.name}`)
  return NextResponse.json(await res.json())
}
