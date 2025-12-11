import PN (toDecimal, pnNumbers, pnShow)
import Control.Lens
import Data.Colour
import Data.Colour.Names
import Data.Default.Class
import Graphics.Rendering.Chart
import Graphics.Rendering.Chart.Backend.Cairo

chart = toRenderable layout
  where
    asLog :: Int -> [Double]
    asLog x = map (log . fromIntegral . toDecimal) (take x pnNumbers)
    pns :: Int -> [(Int, Double)]
    pns x = zip [1..] (asLog x)

    points = plot_points_style .~ filledCircles 2 (opaque red)
              $ plot_points_values .~ pns 1000
              $ plot_points_title .~ "am points"
              $ def

    layout = layout_title .~ "PN Numbers - Log"
           $ layout_plots .~ [toPlot points]
           $ def

-- main = renderableToFile def "build/seq.png" chart

main = print (map toDecimal (take 50 pnNumbers))
