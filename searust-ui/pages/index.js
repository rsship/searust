import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import styles from '@/styles/Home.module.css'
import HomeContainer from '@/pages/Home.js';

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  return (
    <>
      <Head>
        <title>Searust!</title>
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
      <HomeContainer/>
      </main>
    </>
  )
}
