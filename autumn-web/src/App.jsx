import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Navbar from './components/Navbar';
import Footer from './components/Footer';
import Home from './pages/Home';
import Docs from './pages/Docs';
import Contributing from './pages/Contributing';

function App() {
    return (
        <Router>
            <div className="min-h-screen bg-primary bg-grid-pattern selection:bg-accent selection:text-white flex flex-col">
                <main className="relative z-10 antialiased w-full flex-grow flex flex-col">
                    <Navbar />
                    <Routes>
                        <Route path="/" element={<Home />} />
                        <Route path="/docs" element={<Docs />} />
                        <Route path="/docs/contributing" element={<Contributing />} />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    );
}

export default App;
