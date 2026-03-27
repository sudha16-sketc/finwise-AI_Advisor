import { finwiseApi } from '../services/api.js'; 
const MetricsDashboard = () => { 
    const [metrics, setMetrics] = useState(null); 
    const [loading, setLoading] = useState(true); 
    const [error, setError] = useState(null); 
    useEffect(() => { 
        const fetchMetrics = async () => { 
            try { 
                const data = await finwiseApi.getMetrics(); 
                setMetrics(data); setError(null); 
            } catch (err) { 
                setError(err.message || 'Failed to fetch metrics'); 
            } finally { 
                setLoading(false); 
            } 
        
        }; 
        fetchMetrics(); 
        const interval = setInterval(fetchMetrics, 30000); 
        return () => clearInterval(interval); 
    }, 
    []); 
    if (loading) { 
        return ( <div className="text-center py-8 text-slate-500 text-sm animate-pulse"> Loading metrics… </div> ); 
    } // Fail silently — metrics are supplementary 
    if (error || !metrics) return null; 
    const totalUsers = metrics.total_users ?? 0; 
    const activeUsers24h = metrics.active_users_24h ?? 0; 
    const activeUsers7d = metrics.active_users_7d ?? 0; 
    const totalTransactions = metrics.total_transactions ?? 0; 
    const totalAnalyses = metrics.total_analyses ?? 0; 
    const totalConnects = metrics.total_connects ?? 0; 
    const avgActions = metrics.avg_actions_per_user ?? 0; // ✅ Removed Deposits/Withdrawals — replaced with Analyses and Connects 
    const chartData = [ { name: 'Analyses', value: totalAnalyses }, { name: 'Connects', value: totalConnects }, { name: 'Transactions', value: totalTransactions }, ]; 
    return (